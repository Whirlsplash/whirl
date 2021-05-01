// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

#[derive(Debug)]
pub struct RoomIdRequest {
  pub room_name: String,
}
impl RoomIdRequest {
  pub fn parse(data: Vec<u8>) -> Self {
    Self {
      room_name: from_utf8(&data[4..data[0] as usize]).unwrap().to_string(),
    }
  }
}
