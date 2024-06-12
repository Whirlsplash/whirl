// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::{
    constants::Command,
    extendable::{Creatable, Parsable},
  },
  byteorder::{BigEndian, ReadBytesExt},
  bytes::{Buf, BufMut, BytesMut},
};

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

impl Parsable for Teleport {
  fn parse(data: Vec<u8>) -> Self {
    // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      room_id:    data.read_u16::<BigEndian>().unwrap() as i8,
      exit_type:  data.read_u8().unwrap(),
      entry_type: data.read_u8().unwrap(),
      x:          f32::from(data.read_i16::<BigEndian>().unwrap()),
      y:          f32::from(data.read_i16::<BigEndian>().unwrap()),
      z:          f32::from(data.read_i16::<BigEndian>().unwrap()),
      direction:  f32::from(data.read_i16::<BigEndian>().unwrap()),
    }
  }
}

impl Creatable for Teleport {
  fn create_with_short_object_id(&self, short_object_id: u8) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(short_object_id); // ObjId
    command.put_u8(Command::Teleport as u8); // Type

    // Content
    command.put_i16(i16::from(self.room_id));
    command.put_u8(self.exit_type);
    command.put_u8(self.entry_type);
    command.put_i16(self.x as i16);
    command.put_i16(self.y as i16);
    command.put_i16(self.z as i16);
    command.put_i16(self.direction as i16);

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return bytes
    command_as_vec
  }
}
