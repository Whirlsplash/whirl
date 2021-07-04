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
      appear_actor::AppearActor,
      buddy_list::BuddyList,
      property::create::{property_request_as_hub, property_update_as_hub},
      register_object_id::RegisterObjectId,
      session_exit::SessionExit,
      subscribe_distance::SubscribeDistance,
      subscribe_room::SubscribeRoom,
      teleport::Teleport,
      text::Text,
    },
    constants::Command,
    extendable::{Creatable, Parsable, ParsableWithArguments},
  },
  interaction::{peer::Peer, shared::Shared},
  net::{
    constants::{VAR_ERROR, VAR_USERNAME},
    network_property::NetworkProperty,
    property_list::PropertyList,
  },
  packet_parser::parse_commands_from_packet,
  Server,
};

/// Spawn a Hub.
pub struct Hub;
#[async_trait]
impl Server for Hub {
  #[allow(clippy::too_many_lines)]
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

    let mut show_avatar = false;

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
                // TODO: IMPLEMENT
                //
                // This will be interesting to implement as it looks like the
                // AutoServer and RoomServer room ID collectors are linked.
                //
                // There are two possibilities to link these two vectors:
                // create a global, lazy-static vector or add the room ID
                // collector to a shared struct (somehow).
                // Some(Command::RoomIdRq) => {
                //   let room = RoomIdRequest::parse(msg.to_vec());
                //   debug!("received room id request from {}: {}", username, room.room_name);
                //   peer.bytes.get_mut().write_all(&RedirectId {
                //     room_name: (&*room.room_name).to_string(),
                //     room_number: 103,
                //   }.create()).await?;
                // }
                Some(Command::SessExit) => {
                  debug!("received session exit from {}", username);

                  peer.bytes.get_mut().write_all(&SessionExit(PropertyList(vec![
                    NetworkProperty {
                      prop_id: VAR_ERROR,
                      value: "0".to_string(),
                    }
                  ])).create()).await?;
                  trace!("sent session exit to {}", username);

                  break;
                }
                Some(Command::Text) => {
                  let text = Text::parse(msg.to_vec(), &[&username]);
                  debug!("received text from {}: {}", username, text.content);

                  {
                    state.lock().await.broadcast(&Text {
                      sender: (&*username).to_string(),
                      content: text.content.clone(),
                    }.create()).await;
                  }
                  debug!("broadcasted text to hub");

                  match text.content.as_str() {
                    // Makes the friend "fuwn" come online
                    "/friend online fuwn" => {
                      peer.bytes.get_mut().write_all(&[
                        0x09, 0x01, 0x1e, 0x04, 0x66, 0x75, 0x77, 0x6e,
                        0x01,
                      ]).await?;
                    }
                    // Makes the friend "fuwn" go offline
                    "/friend offline fuwn" => {
                      peer.bytes.get_mut().write_all(&[
                        0x09, 0x01, 0x1e, 0x04, 0x66, 0x75, 0x77, 0x6e,
                        0x00,
                      ]).await?;
                    }
                    // Spawns a test avatar with the name "fuwn"
                    "/spawn fuwn" => {
                      show_avatar = true;

                      peer.bytes.get_mut().write_all(&[
                        // REGOBJID
                        0x09, 0xff, 0x0d, 0x04, 0x66, 0x75, 0x77, 0x6e,
                        0x02,

                        // TELEPORT
                        //
                        // It was way more difficult for me to figure out how to
                        // change this command's room ID then it should have
                        // been...
                        //
                        // The room ID in this command is actually the fifth and
                        // sixth bytes (a short), however, the constructor
                        // implies that the FOURTH byte is where the room ID
                        // short begins. I would attempt to change the room ID
                        // of this command (now `0x00, 0x01` or `0x0001`),
                        // modifying the fourth and fifth bytes, effectively
                        // creating a malformed command which would then cause
                        // the client to go unresponsive...
                        0x10, 0xfe, 0x12, 0x02, 0x00, 0x01, 0x00, 0x01,
                        0x00, 0xbf, 0x00, 0xad, 0x00, 0x00, 0x00, 0x2d,

                        // PROPUPD
                        0x16, 0x02, 0x10, 0x05, 0x40, 0x01, 0x0f, 0x61,
                        0x76, 0x61, 0x74, 0x61, 0x72, 0x3a, 0x56, 0x61,
                        0x6d, 0x70, 0x2e, 0x6d, 0x6f, 0x76,
                      ]).await?;
                    }
                    // Puts the test avatar "fuwn" into the asleep action
                    "/sleep fuwn" => {
                      peer.bytes.get_mut().write_all(&[
                        0x12, 0x00, 0x04, 0x66, 0x75, 0x77, 0x6e, 0x10,
                        0x17, 0x40, 0x01, 0x06, 0x61, 0x73, 0x6c, 0x65,
                        0x65, 0x70,
                      ]).await?;
                    }
                    _ => (),
                  }
                }
                Some(Command::Subscrib) => {
                  let subscribe_room = SubscribeRoom::parse(msg[3..].to_vec());
                  debug!("received subscribe room from {}: {:?}",
                    username, subscribe_room);

                  peer.bytes.get_mut().write_all(&RegisterObjectId {
                    long_object_id: "fuwn".to_string(),
                    short_object_id: 2,
                  }.create()).await?;
                  peer.bytes.get_mut().write_all(&AppearActor {
                    short_object_id: 2,
                    room_id: 1,
                    x: 191,
                    y: 173,
                    z: 0,
                    direction: 45,
                  }.create()).await?;
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
                Some(Command::ShortLoc) => {
                  // This is all just test stuff. Once the drone system has been
                  // finalized, this should all be in it's own module (s).
                  if show_avatar {
                    peer.bytes.get_mut().write_all(&[
                      0x29, 0x00, 0x04, 0x66, 0x75, 0x77, 0x6e, 0x10,
                      0x09, 0x80, 0x01, 0x0a, 0x32, 0x30, 0x32, 0x30,
                      0x30, 0x33, 0x31, 0x32, 0x30, 0x30, 0x05, 0x40,
                      0x01, 0x0f, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72,
                      0x3a, 0x56, 0x61, 0x6d, 0x70, 0x2e, 0x6d, 0x6f,
                      0x76,
                    ]).await?;

                    show_avatar = false;
                  }
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
