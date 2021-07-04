// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};
use num_traits::AsPrimitive;

use crate::cmd::{constants::Command, extendable::Creatable};

#[derive(Debug)]
pub struct AppearActor {
  pub short_object_id: i8,
  pub room_id:         u16,
  pub x:               i16,
  pub y:               i16,
  pub z:               i16,
  pub direction:       i16,
}
impl Creatable for AppearActor {
  fn create(&self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0xFE); // ObjId
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(Command::ApprActr as i32 as i8); // Type

    // Content
    command.put_i8(self.short_object_id.as_(): i8); // ObjId, why is it here? Worlds...
    command.put_u16(self.room_id.as_(): u16); // Room ID
    command.put_u16(self.x.as_(): u16); // X
    command.put_u16(self.y.as_(): u16); // Y
    command.put_u16(self.z.as_(): u16); // Z
    command.put_u16(self.direction.as_(): u16); // Direction

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len().as_(): u8 + 1);

    // Return
    command_as_vec
  }
}
