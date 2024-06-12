// Copyright (C) 2021-2024 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::extendable::{Creatable, Parsable},
  bytes::{BufMut, BytesMut},
  std::str::from_utf8,
};

pub struct BuddyList {
  pub buddy:            String,
  pub add_or_logged_on: i8,
}
impl Parsable for BuddyList {
  fn parse(data: Vec<u8>) -> Self {
    Self {
      buddy:            from_utf8(&data[4..data[0] as usize - 1])
        .unwrap()
        .to_string(),
      add_or_logged_on: data[data[0] as usize - 1] as i8,
    }
  }
}
impl Creatable for BuddyList {
  fn create(&self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01); // Object ID
    #[allow(clippy::cast_possible_truncation)]
    command
      .put_i8(crate::cmd::constants::Command::BuddyListNotify as i32 as i8); // Command type
    // Content
    command.put_u8(self.buddy.len() as u8); // Buddy (name) length
    command.put_slice(self.buddy.as_bytes()); // Buddy (name)
    command.put_u8(self.add_or_logged_on as u8); // "Is buddy logged on?"

    let mut command_as_vec = command.to_vec();

    command_as_vec.insert(0, command.len() as u8 + 1);

    command_as_vec
  }
}
