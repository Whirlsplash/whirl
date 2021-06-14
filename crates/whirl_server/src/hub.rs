// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The Hub functions as a
//! [`RoomServer`](http://dev.worlds.net/private/GammaDocs/WorldServer.html#AutoServer).
//!
//! A `RoomServer` is responsible for handling just about every request from the
//! client after they have been redirected to a room (Hub) and finished their
//! business with the Distributor (`AutoServer`).

use std::{error::Error, net::SocketAddr, sync::Arc};

use num_traits::cast::AsPrimitive;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use whirl_config::Config;

use crate::{
  cmd::{
    commands::{
      action::create,
      buddy_list::BuddyList,
      property::create::{property_request_as_hub, property_update_as_hub},
      subscribe_distance::SubscribeDistance,
      subscribe_room::SubscribeRoom,
      teleport::Teleport,
      text::Text,
    },
    constants::Command,
    extendable::{Creatable, Parsable, ParsableWithArguments},
  },
  interaction::{peer::Peer, shared::Shared},
  net::constants::VAR_USERNAME,
  packet_parser::parse_commands_from_packet,
  Server,
};

/// Spawn a Hub.
pub struct Hub;
#[async_trait]
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
          // trace!("got peer activity: {:?}", &msg);
          peer.bytes.get_mut().write_all(&msg).await?;
        }
        result = peer.bytes.next() => match result {
          Some(Ok(msg)) => {
            // trace!("got some bytes: {:?}", &msg);
            for msg in parse_commands_from_packet(msg) {
              match num_traits::FromPrimitive::from_i32(msg.get(2).unwrap().to_owned().as_(): i32) {
                Some(Command::PropReq) => {
                  debug!("received property request from client");

                  peer.bytes.get_mut()
                    .write_all(&property_update_as_hub()).await?;
                  trace!("sent property update to client");
                }
                Some(Command::SessInit) => {
                  username = (*crate::net::property_list::PropertyList::from_bytes(msg[3..]
                    .to_vec())
                    .find(VAR_USERNAME)).value.to_string();

                  debug!("received session initialization from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&property_request_as_hub()).await?;
                  trace!("sent property request to {}", username);
                }
                Some(Command::PropSet) => {
                  debug!("received property set from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&Text {
                    sender: Config::get().whirlsplash.worldsmaster_username,
                    content: Config::get().distributor.worldsmaster_greeting,
                  }.create()).await?;
                  peer.bytes.get_mut()
                    .write_all(&create()).await?;
                  trace!("sent text to {}", username);
                }
                Some(Command::BuddyListUpdate) => {
                  let buddy = BuddyList::parse(msg.to_vec());
                  debug!("received buddy list update from {}: {}", username, buddy.buddy);
                  peer.bytes.get_mut().write_all(&buddy.create()).await?;
                  trace!("sent buddy list notify to {}: {}", username, buddy.buddy);
                }
                // TODO: Figure out if this is actually even needed.
                // Some(Command::RoomIdRq) => {
                //   let room = RoomIdRequest::parse(msg.to_vec());
                //   debug!("received room id request from {}: {}", username, room.room_name);
                //   trace!("{:?}", create_room_id_request(&room.room_name, 0x00));
                // }
                Some(Command::SessExit) => {
                  debug!("received session exit from {}", username); break;
                }
                Some(Command::Text) => {
                  let text = Text::parse(msg.to_vec(), &[&username]);
                  debug!("received text from {}: {}", username, text.content);

                  {
                    state.lock().await.broadcast(&Text {
                      sender: (&*username).to_string(),
                      content: text.content,
                    }.create()).await;
                  }
                  debug!("broadcasted text to hub");
                }
                Some(Command::Subscrib) => {
                  let subscribe_room = SubscribeRoom::parse(msg[3..].to_vec());
                  debug!("received subscribe room from {}: {:?}",
                    username, subscribe_room);
                }
                Some(Command::SubDist) => {
                  let subscribe_distance = SubscribeDistance::parse(msg[3..].to_vec());
                  debug!("received subscribe distance from {}: {:?}",
                    username, subscribe_distance);
                }
                Some(Command::Teleport) => {
                  let teleport = Teleport::parse(msg[3..].to_vec());
                  debug!("received teleport from {}: {:?}",
                    username, teleport);
                }
                _ => (),
              }
            }
          }
          Some(Err(e)) => {
            error!("error while processing message (s): {}", e); break;
          }
          None => {
            trace!("nothing"); break;
          },
        }
      }
    }

    // Deregister client
    debug!("de-registering client");
    {
      state.lock().await.peers.remove(&count.to_string());
    }
    debug!("de-registered client");

    Ok(())
  }
}
