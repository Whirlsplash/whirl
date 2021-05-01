// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct SubscribeRoom {
  pub room_number: i8,
  pub x:           f32,
  pub y:           f32,
  pub z:           f32,
  pub distance:    f32,
}
impl SubscribeRoom {
  pub fn parse(data: Vec<u8>) -> Self {
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      room_number: data.read_i16::<BigEndian>().unwrap() as i8,
      x:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
      y:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
      z:           data.read_i16::<BigEndian>().unwrap() as i8 as f32,
      distance:    data.read_i16::<BigEndian>().unwrap() as i8 as f32, // + 100
    }
  }
}
