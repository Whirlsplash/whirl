// Copyleft (ɔ) 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

pub fn parse_room_id_request(data: Vec<u8>) -> String {
  from_utf8(&data[4..data[0] as usize]).unwrap().to_string()
}
