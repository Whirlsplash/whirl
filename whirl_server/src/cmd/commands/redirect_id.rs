// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};
use whirl_config::Config;

use crate::cmd::{constants::REDIRID, extendable::Creatable};

#[derive(Debug)]
pub struct RedirectId {
  pub room_name:   String,
  pub room_number: i8,
}
impl Creatable for RedirectId {
  fn create(self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01); // ObjId
    command.put_u8(REDIRID as u8); // Type

    // Content
    command.put_u8(self.room_name.len() as u8); // Room name length
    command.put_slice(self.room_name.as_bytes()); // Room name
                                                  // command.put_u8(0x00); // Unimplemented byte (?)
                                                  // command.put_u8(room_id); // Room ID
    command.put_u16(self.room_number as u16); // Room ID

    // IP
    for byte in Config::get().whirlsplash.ip.split('.') {
      command.put_u8(byte.parse::<u8>().unwrap());
    }
    command.put_u16(Config::get().hub.port as u16); // Port

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return
    command_as_vec
  }
}
