// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use bytes::{BufMut, BytesMut};

use crate::cmd::extendable::{Creatable, Parsable};

pub struct BuddyList {
  pub buddy: String,
  pub add:   i8,
}
impl Parsable for BuddyList {
  fn parse(data: Vec<u8>) -> Self {
    Self {
      buddy: from_utf8(&data[4..data[0] as usize - 1])
        .unwrap()
        .to_string(),

      // Get the last byte
      add: data[data[0] as usize - 1] as i8,
    }
  }
}
impl Creatable for BuddyList {
  fn create(&self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01); // ObjId
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(crate::cmd::constants::Command::BuddyListNotify as i32 as i8); // Type

    // Content
    command.put_u8(self.buddy.len() as u8); // Buddy (name) length
    command.put_slice(self.buddy.as_bytes()); // Buddy (name)
    command.put_u8(self.add as u8); // "Is buddy logged on?" (?)

    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    command_as_vec
  }
}
