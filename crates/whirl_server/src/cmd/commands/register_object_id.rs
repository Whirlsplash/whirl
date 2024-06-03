// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::{constants::Command, extendable::Creatable},
  bytes::{BufMut, BytesMut},
};

#[derive(Debug)]
pub struct RegisterObjectId {
  pub long_object_id:  String,
  pub short_object_id: i8,
}
impl Creatable for RegisterObjectId {
  fn create(&self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0xFF); // ObjId
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(Command::RegObjId as i32 as i8); // Type

    // Content
    command.put_u8(self.long_object_id.len() as u8); // Long object ID length
    command.put_slice(self.long_object_id.as_bytes()); // Long object ID
    command.put_i8(self.short_object_id); // Short object ID

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return
    command_as_vec
  }
}
