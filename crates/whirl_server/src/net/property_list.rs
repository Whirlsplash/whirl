// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::{BufMut, BytesMut};
use num_traits::AsPrimitive;

use crate::{
  cmd::constants::Command,
  net::{
    constants::{PROPACCESS_POSSESS, PROPFLAG_DBSTORE},
    network_property::NetworkProperty,
  },
};

pub struct PropertyList(pub Vec<crate::net::network_property::NetworkProperty>);
impl PropertyList {
  /// Convert a `PropertyList` to a ready-to-be sent command.
  pub fn as_bytes(&mut self, command_id: i32, obj_id: i32) -> Vec<u8> {
    let mut command = BytesMut::new();
    let property_list = &mut self.0;

    // Iterate over all network properties
    loop {
      // Check if there are any properties left
      trace!("props left: {}", property_list.len());
      if property_list.is_empty() {
        break;
      }

      let property = &property_list[0]; // Property we are currently iterating over
      trace!("current prop: {}:{}", property.prop_id, property.value);

      command.put_u8(property.prop_id.as_(): u8); // Property ID

      // NOTE: THIS IS SUPER BAD DO NOT DO THIS! But it works!
      if command_id == Command::PropUpd as i32 {
        command.put_u8(PROPFLAG_DBSTORE.as_(): u8); // Flag (s)
        command.put_u8(PROPACCESS_POSSESS.as_(): u8); // Access
      }

      command.put_u8(property.value.len().as_(): u8); // Property UTF-8 Length
      command.put_slice(property.value.as_bytes()); // Property UTF-8

      property_list.reverse();
      property_list.pop();
      property_list.reverse();
    }

    // Convert to vector and insert the header
    let mut command_as_vec = command.to_vec();

    command_as_vec.insert(0, command_id.as_(): u8); // Command ID
    command_as_vec.insert(0, obj_id.as_(): u8); // ObjId
    command_as_vec.insert(0, command.len().as_(): u8 + 3); // Data length

    // Return bytes
    command_as_vec
  }

  /// Find and return a reference to a `NetworkProperty` within the associated
  /// `PropertyList`.
  pub fn find(&self, property: i32) -> &NetworkProperty {
    self.0.iter().find(|i| i.prop_id == property).unwrap()
  }

  /// Iterate over a property list in it's original, encoded, byte form
  /// (`Vec<u8>`), and return a list of human-readable network properties
  /// (`PropertyList`).
  pub fn from_bytes(mut data: Vec<u8>) -> Self {
    let mut property_list = vec![];

    // Iterate over all network properties
    loop {
      // Check if any commands are present
      if data.len() <= 2 {
        break;
      }
      trace!("iteration: {:?}", data);
      // if data[0] == 0 {
      //   break;
      // }

      let property_length = data[1] + 2;
      property_list.push(crate::net::network_property::NetworkProperty {
        prop_id: i32::from(data[0]),
        value:   std::str::from_utf8(&data[2..data[1] as usize + 2])
          .unwrap()
          .to_string(),
      });

      // Remove current property from the network property
      data = data[property_length as usize..].to_vec();
    }

    // Return the human-readable network property
    Self(property_list)
  }
}
