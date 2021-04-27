// Copyleft (ɔ) 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};

pub fn create_action() -> Vec<u8> {
  let mut command = BytesMut::new();

  command.put_slice(&[
    0x01, 0x11, 0x00, 0x05, 0x54, 0x52, 0x41, 0x44, 0x45, 0x07, 0x26, 0x7c, 0x2b, 0x69, 0x6e, 0x76,
    0x3e,
  ]);

  // Convert to vector and insert the length
  let mut command_as_vec = command.to_vec();
  command_as_vec.insert(0, command.len() as u8 + 1);

  // Return bytes
  command_as_vec
}
