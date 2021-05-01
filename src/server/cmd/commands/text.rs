// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use bytes::{BufMut, BytesMut};

use crate::server::cmd::constants::TEXT;

pub struct Text {
  pub sender:  String,
  pub content: String,
}
impl Text {
  pub fn parse(data: Vec<u8>, username: &str) -> Self {
    Self {
      sender:  username.to_string(),
      content: from_utf8(&data[6..]).unwrap().to_string(),
    }
  }

  pub fn create(self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01);
    command.put_i8(TEXT as i8);

    // Content
    // TODO: Find a way to parse ObjIds.
    //  The below byte is suspected to be the sender's short ObjId.
    command.put_i8(0x00);

    command.put_u8(self.sender.len() as u8);
    command.put_slice(self.sender.as_bytes());
    command.put_u8(self.content.len() as u8);
    command.put_slice(self.content.as_bytes());

    // Convert to vector and insert the length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return bytes
    command_as_vec
  }
}
