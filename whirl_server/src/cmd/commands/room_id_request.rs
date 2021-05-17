// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use crate::cmd::extendable::Parsable;

#[derive(Debug)]
pub struct RoomIdRequest {
  pub room_name: String,
}
impl Parsable for RoomIdRequest {
  fn parse(data: Vec<u8>) -> Self {
    Self {
      room_name: from_utf8(&data[4..data[0] as usize]).unwrap().to_string(),
    }
  }
}
