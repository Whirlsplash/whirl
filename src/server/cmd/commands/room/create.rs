// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};

use crate::{config::Config, server::cmd::constants::REDIRID};

pub fn create_room_id_request(room: &str, room_id: u8) -> Vec<u8> {
  let mut command = BytesMut::new();

  // Header
  command.put_u8(0x01); // ObjId
  command.put_u8(REDIRID as u8); // Type

  // Content
  command.put_u8(room.len() as u8); // Room name length
  command.put_slice(room.as_bytes()); // Room name
                                      // command.put_u8(0x00); // Unimplemented byte (?)
                                      // command.put_u8(room_id); // Room ID
  command.put_u16(room_id as u16); // Room ID

  // IP
  for byte in "0.0.0.0".split('.') {
    command.put_u8(byte.parse::<u8>().unwrap());
  }
  command.put_u16(Config::get().unwrap().hub.port as u16); // Port

  // Length
  let mut command_as_vec = command.to_vec();
  command_as_vec.insert(0, command.len() as u8 + 1);

  // Return
  command_as_vec
}
