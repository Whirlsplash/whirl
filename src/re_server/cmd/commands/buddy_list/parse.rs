use std::str::from_utf8;

use crate::re_server::cmd::commands::buddy_list::structure::Buddy;

pub fn parse_buddy_list_update(data: Vec<u8>) -> Buddy {
  Buddy {
    buddy: from_utf8(&data[4..data[0] as usize - 1])
      .unwrap()
      .to_string(),

    // Get the last byte
    add: data[data[0] as usize - 1],
  }
}
