// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use bytes::{BufMut, BytesMut};

use crate::server::cmd::constants::BUDDYLISTNOTIFY;

#[derive(Clone)]
pub struct BuddyList {
  pub buddy: String,
  pub add:   i8,
}
impl BuddyList {
  pub fn parse(data: Vec<u8>) -> Self {
    Self {
      buddy: from_utf8(&data[4..data[0] as usize - 1])
        .unwrap()
        .to_string(),

      // Get the last byte
      add: data[data[0] as usize - 1] as i8,
    }
  }

  pub fn create(self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01); // ObjId
    command.put_i8(BUDDYLISTNOTIFY as i8); // Type

    // Content
    command.put_u8(self.buddy.len() as u8); // Buddy (name) length
    command.put_slice(self.buddy.as_bytes()); // Buddy (name)
    command.put_u8(self.add as u8); // "Is buddy logged on?" (?)

    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    command_as_vec
  }
}
