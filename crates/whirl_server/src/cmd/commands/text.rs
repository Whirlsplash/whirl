// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use bytes::{BufMut, BytesMut};
use num_traits::AsPrimitive;

use crate::cmd::{
  constants::Command,
  extendable::{Creatable, ParsableWithArguments},
};

pub struct Text {
  pub sender:  String,
  pub content: String,
}
impl Creatable for Text {
  fn create(&self) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(0x01);
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(Command::Text as i32 as i8);

    // Content
    // TODO: Find a way to parse ObjIds.
    //
    // The below byte is suspected to be the sender's short ObjId.
    command.put_i8(0x00);

    command.put_u8(self.sender.len().as_(): u8);
    command.put_slice(self.sender.as_bytes());
    command.put_u8(self.content.len().as_(): u8);
    command.put_slice(self.content.as_bytes());

    // Convert to vector and insert the length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len().as_(): u8 + 1);

    // Return bytes
    command_as_vec
  }
}
impl ParsableWithArguments for Text {
  /// The first and only element of `args` *should* be the username of the
  /// sender.
  ///
  /// There isn't anything currently stopping someone from passing some other
  /// value so that might be annoying at times.
  ///
  /// Realistically, this method is mostly static so the username will *always*
  /// be passed properly unless someone intentionally commits breaking changes
  /// on purpose regarding what is passed to to this method where called.
  ///
  /// It would be neat to have some sort of ability to statically check if the
  /// `args` argument contains x number of elements at compile time or
  /// something of the sort but the Rust RFC is probably not focused on that.
  ///
  /// So, right now, trust is in the developers' hands to make sure to pass the
  /// right — number — of elements to `args`.
  fn parse(data: Vec<u8>, args: &[&str]) -> Self {
    Self {
      sender:  args[0].to_string(),
      content: from_utf8(&data[6..]).unwrap().to_string(),
    }
  }
}
