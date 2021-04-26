// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};

use crate::server::cmd::{commands::buddy_list::structure::Buddy, constants::BUDDYLISTNOTIFY};

pub fn create_buddy_list_notify(buddy: &Buddy) -> Vec<u8> {
  let mut command = BytesMut::new();

  // Header
  command.put_u8(0x01); // ObjId
  command.put_u8(BUDDYLISTNOTIFY as u8); // Type

  // Content
  command.put_u8(buddy.buddy.len() as u8); // Buddy (name) length
  command.put_slice(buddy.buddy.as_bytes()); // Buddy (name)
  command.put_u8(buddy.add); // "Is buddy logged on?" (?)

  let mut command_as_vec = command.to_vec();
  command_as_vec.insert(0, command.len() as u8 + 1);

  command_as_vec
}
