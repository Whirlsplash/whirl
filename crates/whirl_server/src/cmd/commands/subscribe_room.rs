// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::extendable::Parsable,
  byteorder::{BigEndian, ReadBytesExt},
  bytes::{Buf, BytesMut},
};

#[derive(Debug)]
pub struct SubscribeRoom {
  pub room_number: i8,
  pub x:           f32,
  pub y:           f32,
  pub z:           f32,
  pub distance:    f32,
}
impl Parsable for SubscribeRoom {
  fn parse(data: Vec<u8>) -> Self {
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      room_number: data.read_i16::<BigEndian>().unwrap() as i8,
      x:           f32::from(data.read_i16::<BigEndian>().unwrap() as i8),
      y:           f32::from(data.read_i16::<BigEndian>().unwrap() as i8),
      z:           f32::from(data.read_i16::<BigEndian>().unwrap() as i8),
      distance:    f32::from(data.read_i16::<BigEndian>().unwrap() as i8), /* + 100 */
    }
  }
}
