// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

use crate::re_server::cmd::commands::subscribe::structure::SubscribeRoom;

/// TODO: The functionality of this function has not been tested... TEST IT!
pub fn parse_subscribe_room(data: Vec<u8>) -> SubscribeRoom {
  // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
  let mut data = BytesMut::from(data.as_slice()).reader();

  SubscribeRoom {
    room_number: data.read_i16::<BigEndian>().unwrap(),
    distance:    data.read_i16::<BigEndian>().unwrap(),
    x:           data.read_i16::<BigEndian>().unwrap(),
    y:           data.read_i16::<BigEndian>().unwrap(),
    z:           data.read_i16::<BigEndian>().unwrap(),
  }
}
