// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

use crate::cmd::extendable::Parsable;

#[derive(Debug)]
pub struct SubscribeDistance {
  pub distance:    i16,
  pub room_number: i16,
}
impl Parsable for SubscribeDistance {
  fn parse(data: Vec<u8>) -> Self {
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      distance:    data.read_i16::<BigEndian>().unwrap(),
      room_number: data.read_i16::<BigEndian>().unwrap(),
    }
  }
}
