// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

use crate::server::cmd::commands::subscribe_room::structure::SubscribeRoom;

/// TODO: The functionality of this function has not been tested... TEST IT!
pub fn parse_subscribe_room(data: Vec<u8>) -> SubscribeRoom {
  // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
  let mut data = BytesMut::from(data.as_slice()).reader();

  SubscribeRoom {
    room_number: data.read_i16::<BigEndian>().unwrap() as i8,
    x:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
    y:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
    z:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
    distance:    data.read_i16::<BigEndian>().unwrap() as i8 as f32, // + 100
  }
}