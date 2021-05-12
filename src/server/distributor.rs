// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The distributor functions as bare-minimal
//! [AutoServer](http://dev.worlds.net/private/GammaDocs/WorldServer.html#AutoServer).
//!
//! It intercepts a client and distributes it to a
//! [RoomServer](http://dev.worlds.net/private/GammaDocs/WorldServer.html#RoomServer).
//!
//! This is not meant to be a high performant section of code as the distributor
//! is only meant to handle the initial and brief session initialization of the
//! client.

use std::{error::Error, net::SocketAddr, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

use crate::{
  config::Config,
  server::{
    cmd::{
      commands::{
        action::create_action,
        buddy_list::BuddyList,
        property::{
          create::{create_property_request_as_distributor, create_property_update_as_distributor},
          parse::find_property_in_property_list,
        },
        redirect_id::RedirectId,
        room_id_request::RoomIdRequest,
        text::Text,
      },
      constants::*,
      extendable::{Creatable, Parsable},
    },
    interaction::{peer::Peer, shared::Shared},
    net::{constants::VAR_USERNAME, property_parser::parse_network_property},
    packet_parser::parse_commands_from_packet,
    Server,
  },
};

pub struct Distributor;
#[async_trait]
impl Server for Distributor {
  async fn handle(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    _address: SocketAddr,
    count: usize,
  ) -> Result<(), Box<dyn Error>> {
    let bytes = BytesCodec::new().framed(stream);
    let mut peer = Peer::new(state.clone(), bytes, count.to_string()).await?;
    let mut room_ids = vec![];
    let mut username = String::from("unknown");

    loop {
      tokio::select! {
        Some(msg) = peer.rx.recv() => {
          peer.bytes.get_mut().write_all(&msg).await?;
        }
        result = peer.bytes.next() => match result {
          Some(Ok(msg)) => {
            for msg in parse_commands_from_packet(msg) {
              match msg.get(2).unwrap().to_owned() as i32 {
                PROPREQ => {
                  trace!("received property request from client");

                  peer.bytes.get_mut()
                    .write_all(&create_property_update_as_distributor()).await?;
                  trace!("sent property update to client");
                }
                SESSINIT => {
                  username = find_property_in_property_list(
                    &parse_network_property(msg[3..].to_vec()),
                    VAR_USERNAME,
                  ).value.clone();

                  trace!("received session initialization from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&create_property_request_as_distributor()).await?;
                  trace!("sent property request to {}", username);
                }
                PROPSET => {
                  trace!("received property set from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&Text {
                    sender: Config::get().whirlsplash.worldsmaster_username,
                    content: Config::get().distributor.worldsmaster_greeting,
                  }.create()).await?;
                  peer.bytes.get_mut()
                    .write_all(&create_action()).await?;
                  trace!("sent text to {}", username);
                }
                BUDDYLISTUPDATE => {
                  let buddy = BuddyList::parse(msg.to_vec());
                  trace!("received buddy list update from {}: {}", username, buddy.buddy);
                  peer.bytes.get_mut().write_all(&BuddyList {
                    ..buddy.clone()
                  }.create()).await?;
                  trace!("sent buddy list notify to {}: {}", username, buddy.buddy);
                }
                ROOMIDRQ => {
                  let room = RoomIdRequest::parse(msg.to_vec());
                  trace!("received room id request from {}: {}", username, &room.room_name);

                  let room_id;
                  if !room_ids.contains(&room.room_name) {
                    room_ids.push(room.room_name.clone());
                    room_id = room_ids.iter().position(|r| r == &room.room_name).unwrap();
                    debug!("inserted room: {}", room.room_name);
                  } else {
                    let position = room_ids.iter().position(|r| r == &room.room_name).unwrap();
                    debug!("found room: {}", room.room_name);
                    room_id = position;
                  }

                  peer.bytes.get_mut().write_all(&RedirectId {
                    room_name: room.room_name.clone(),
                    room_number: room_id as i8,
                  }.create()).await?;
                  trace!("sent redirect id to {}: {}", username, room.room_name);
                }
                SESSEXIT => {
                  trace!("received session exit from {}", username); break;
                }
                _ => (),
              }
            }
          }
          Some(Err(e)) => {
            error!("error while processing message (s): {}", e); break;
          }
          None => break,
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
