// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The Distributor functions as bare-minimal
//! [`AutoServer`](http://dev.worlds.net/private/GammaDocs/WorldServer.html#AutoServer).
//!
//! The Distributor intercepts a client on initial connection and distributes
//! it to a
//! [`RoomServer`](http://dev.worlds.net/private/GammaDocs/WorldServer.html#RoomServer).
//!
//! This is not meant to be a high focus module as the Distributor is only
//! meant to handle the initial and brief session initialization of the client.

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
      property::create::{property_request_as_distributor, property_update_as_distributor},
      redirect_id::RedirectId,
      room_id_request::RoomIdRequest,
      session_exit::SessionExit,
      text::Text,
    },
    constants::Command,
    extendable::{Creatable, Parsable},
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

/// Spawn a Distributor.
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
              match num_traits::FromPrimitive::from_i32(msg.get(2).unwrap().to_owned().as_(): i32) {
                Some(Command::PropReq) => {
                  debug!("received property request from client");

                  peer.bytes.get_mut()
                    .write_all(&property_update_as_distributor()).await?;
                  trace!("sent property update to client");
                }
                Some(Command::SessInit) => {
                  username = (*crate::net::property_list::PropertyList::from_bytes(msg[3..]
                    .to_vec())
                    .find(VAR_USERNAME)).value.to_string();

                  debug!("received session initialization from {}", username);

                  peer.bytes.get_mut()
                    .write_all(&property_request_as_distributor()).await?;
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
                Some(Command::RoomIdRq) => {
                  let room = RoomIdRequest::parse(msg.to_vec());
                  debug!("received room id request from {}: {}", username, &room.room_name);

                  let room_id;
                  if room_ids.contains(&room.room_name) {
                    let position = room_ids.iter().position(|r| r == &room.room_name).unwrap();
                    trace!("found room: {}", room.room_name);
                    room_id = position;
                  } else {
                    room_ids.push((&*room.room_name).to_string());
                    room_id = room_ids.iter().position(|r| r == &room.room_name).unwrap();
                    trace!("inserted room: {}", room.room_name);
                  }

                  peer.bytes.get_mut().write_all(&RedirectId {
                    room_name: (&*room.room_name).to_string(),
                    room_number: room_id.as_(): i8,
                  }.create()).await?;
                  trace!("sent redirect id to {}: {}", username, room.room_name);
                }
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
                _ => {},
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
    debug!("de-registering client");
    {
      state.lock().await.peers.remove(&count.to_string());
    }
    debug!("de-registered client");

    Ok(())
  }
}
