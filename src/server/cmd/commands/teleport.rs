// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct Teleport {
  pub room_id:    i8,
  pub exit_type:  u8,
  pub entry_type: u8,
  pub x:          f32, // i16
  pub y:          f32,
  pub z:          f32,
  pub direction:  f32,
}
impl Teleport {
  pub fn parse(data: Vec<u8>) -> Self {
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      room_id:    data.read_u16::<BigEndian>().unwrap() as i8,
      exit_type:  data.read_u8().unwrap(),
      entry_type: data.read_u8().unwrap(),
      x:          data.read_i16::<BigEndian>().unwrap() as f32,
      y:          data.read_i16::<BigEndian>().unwrap() as f32,
      z:          data.read_i16::<BigEndian>().unwrap() as f32,
      direction:  data.read_i16::<BigEndian>().unwrap() as f32,
    }
  }
}
