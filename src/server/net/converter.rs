// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};

use crate::server::{
  cmd::constants::PROPUPD,
  net::{
    constants::{PROPACCESS_POSSESS, PROPFLAG_DBSTORE},
    structure::NetworkProperty,
  },
};

pub fn property_list_to_bytes(
  command_id: i32,
  obj_id: i32,
  mut property_list: Vec<NetworkProperty>,
) -> Vec<u8> {
  let mut command = BytesMut::new();

  // Iterate over all network properties
  loop {
    // Check if there are any properties left
    debug!("props left: {}", property_list.len());
    if property_list.is_empty() {
      break;
    }

    let property = &property_list[0]; // Property we are currently iterating over
    debug!("current prop: {}:{}", property.prop_id, property.value);

    command.put_u8(property.prop_id as u8); // Property ID

    // NOTE: THIS IS SUPER BAD DO NOT DO THIS! But it works!
    if command_id == PROPUPD {
      command.put_u8(PROPFLAG_DBSTORE as u8); // Flag (s)
      command.put_u8(PROPACCESS_POSSESS as u8); // Access
    }

    command.put_u8(property.value.len() as u8); // Property UTF-8 Length
    command.put_slice(property.value.as_bytes()); // Property UTF-8

    property_list.reverse();
    property_list.pop();
    property_list.reverse();
  }

  // Convert to vector and insert the header
  let mut command_as_vec = command.to_vec();

  command_as_vec.insert(0, command_id as u8); // Command ID
  command_as_vec.insert(0, obj_id as u8); // ObjId
  command_as_vec.insert(0, command.len() as u8 + 3); // Data length

  // Return bytes
  command_as_vec
}
