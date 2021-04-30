// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

use crate::server::cmd::commands::subscribe_distance::structure::SubscribeDistance;

pub fn parse_subscribe_distance(data: Vec<u8>) -> SubscribeDistance {
  // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
  let mut data = BytesMut::from(data.as_slice()).reader();

  SubscribeDistance {
    distance:    data.read_i16::<BigEndian>().unwrap(),
    room_number: data.read_i16::<BigEndian>().unwrap(),
  }
}
