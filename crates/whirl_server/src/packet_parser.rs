// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use bytes::BytesMut;

/// Read all commands from the given `buffer`.
///
/// # Process
/// 1. Read a command from `buffer` based on it's first byte.
/// 2. Push command to `commands`.
/// 3. Remove command from `buffer`.
/// 4. Iterate and do this for the remaining commands within `buffer`.
pub fn parse_commands_from_packet(mut buffer: BytesMut) -> Vec<BytesMut> {
  let mut commands: Vec<BytesMut> = Vec::new();
  trace!("initial buffer: {:?}, length: {}", buffer, buffer.len());

  let data_length = buffer.first().unwrap().to_owned() as usize;
  if buffer.len() > data_length {
    loop {
      trace!("loop: {:?}, length: {}", buffer, buffer.len());
      let command_length = buffer.first().unwrap().to_owned() as usize;
      commands.push(BytesMut::from(buffer.get(0..command_length).unwrap()));

      // Remove command from buffer
      buffer = buffer.split_off(command_length);

      // Check if any more commands are present
      if buffer.is_empty() {
        break;
      }
    }
  } else {
    // There will always be at least one command, push it.
    commands.push(BytesMut::from(buffer.get(0..data_length).unwrap()));
  }

  commands // Return command (s)
}
