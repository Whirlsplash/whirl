//! The hub functions as a
//! [RoomServer](http://dev.worlds.net/private/GammaDocs/WorldServer.html#AutoServer).
//!
//! The RoomServer is responsible for handling just about every request from the
//! client after they have been redirected to a room (hub).

use std::{error::Error, net::SocketAddr, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

use crate::{
  config::get_config,
  re_server::{
    cmd::{
      commands::{
        action::create_action,
        buddy_list::{create::create_buddy_list_notify, parse::parse_buddy_list_update},
        property::{
          create::{create_property_request_as_hub, create_property_update_as_hub},
          parse::find_property_in_property_list,
        },
        room::{create::create_room_id_request, parse::parse_room_id_request},
        text::{create::create_text, parse::parse_text, structure::Text},
      },
      constants::*,
    },
    interaction::{peer::Peer, shared::Shared},
    net::{constants::VAR_USERNAME, property_parser::parse_network_property},
    packet_parser::parse_commands_from_packet,
    server::Server,
  },
};

pub struct Hub;
#[async_trait::async_trait]
impl Server for Hub {
  async fn handle(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    _address: SocketAddr,
    count: usize,
  ) -> Result<(), Box<dyn Error>> {
    let bytes = BytesCodec::new().framed(stream);
    let mut peer = Peer::new(state.clone(), bytes, count.to_string()).await?;
    // let mut room_ids = vec![];
    let mut username = String::from("unknown");

    loop {
      tokio::select! {
        Some(msg) = peer.rx.recv() => {
          dbg!("got peer activity: {:?}", &msg);
          peer.bytes.get_mut().write_all(&msg).await?;
        }
        result = peer.bytes.next() => match result {
          Some(Ok(msg)) => {
            dbg!("got some bytes: {:?}", &msg);
            for msg in parse_commands_from_packet(msg) {
              match msg.get(2).unwrap().to_owned() as i32 {
                PROPREQ => {
                  trace!("received property request from client");

                  peer.bytes.get_mut()
                    .write_all(&create_property_update_as_hub()).await?;
                  trace!("sent property update to client");
                }
                SESSINIT => {
                  username = find_property_in_property_list(
                    &parse_network_property(msg[3..].to_vec()),
                    VAR_USERNAME,
                  ).value.clone();

                  trace!("received session initialization from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&create_property_request_as_hub()).await?;
                  trace!("sent property request to {}", username);
                }
                PROPSET => {
                  trace!("received property set from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&create_text(Text {
                    sender: get_config()?.worldsmaster_username,
                    content: get_config()?.worldsmaster_greeting,
                  })).await?;
                  peer.bytes.get_mut()
                    .write_all(&create_action()).await?;
                  trace!("sent text to {}", username);
                }
                BUDDYLISTUPDATE => {
                  let buddy = parse_buddy_list_update(msg.to_vec());
                  trace!("received buddy list update from {}: {}", username, buddy.buddy);
                  peer.bytes.get_mut()
                    .write_all(&create_buddy_list_notify(&buddy)).await?;
                  trace!("sent buddy list notify to {}: {}", username, buddy.buddy);
                }
                ROOMIDRQ => {
                  let room = parse_room_id_request(msg.to_vec());
                  trace!("received room id request from {}: {}", username, room);
                  debug!("{:?}", create_room_id_request(&room, 0x04));
                }
                SESSEXIT => {
                  trace!("received session exit from {}", username); break;
                }
                TEXT => {
                  let text = parse_text(msg.to_vec(), &username);
                  trace!("received text from {}:{}", username, text.content);

                  {
                    state.lock().await.broadcast(&create_text(Text {
                      sender: username.clone(),
                      content: text.content,
                    })).await;
                  }
                  trace!("broadcasted text to hub");
                }
                _ => (),
              }
            }
          }
          Some(Err(e)) => {
            error!("error while processing message (s): {}", e); break;
          }
          None => {
            debug!("nothing"); break;
          },
        }
      }
    }

    // Deregister client
    trace!("de-registering client");
    {
      state.lock().await.peers.remove(&count.to_string());
    }
    trace!("de-registered client");

    Ok(())
  }
}