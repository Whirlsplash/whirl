// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::{constants::Command, extendable::Creatable},
  bytes::{BufMut, BytesMut},
};

#[derive(Debug)]
pub struct AppearActor {
  pub room_id:   u16,
  pub x:         i16,
  pub y:         i16,
  pub z:         i16,
  pub direction: i16,
}

impl Creatable for AppearActor {
  fn create_with_short_object_id(&self, short_object_id: u8) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(short_object_id); // ObjId
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(Command::ApprActr as i32 as i8); // Type

    // Content
    command.put_u16(self.room_id); // Room ID
    command.put_i16(self.x); // X
    command.put_i16(self.y); // Y
    command.put_i16(self.z); // Z
    command.put_i16(self.direction); // Direction

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return
    command_as_vec
  }
}
