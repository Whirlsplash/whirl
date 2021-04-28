// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};

use crate::server::cmd::{commands::text::structure::Text, constants::TEXT};

pub fn create_text(text: Text) -> Vec<u8> {
  let mut command = BytesMut::new();

  // Header
  command.put_u8(0x01);
  command.put_u8(TEXT as u8);

  // Content
  // The fourth and fifth elements are presumed to be interpreted as a short by
  // the client, however, usernames aren't (?) allowed to be long enough that
  // they reach a number high enough to be converted to a short.
  command.put_u8(0x00);
  command.put_u8(text.sender.len() as u8);
  command.put_slice(text.sender.as_bytes());
  command.put_u8(text.content.len() as u8);
  command.put_slice(text.content.as_bytes());

  // Convert to vector and insert the length
  let mut command_as_vec = command.to_vec();
  command_as_vec.insert(0, command.len() as u8 + 1);

  // Return bytes
  command_as_vec
}
