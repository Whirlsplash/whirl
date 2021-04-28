// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use crate::server::cmd::commands::text::structure::Text;

pub fn parse_text(data: Vec<u8>, username: &str) -> Text {
  Text {
    sender:  username.to_string(),
    content: from_utf8(&data[6..]).unwrap().to_string(),
  }
}
