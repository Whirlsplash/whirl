// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use crate::cmd::structure::Command;

/// Iterate over a command set in the from of bytes (Vec<u8>) and return a list
/// of human-readable commands.
fn _parse_command_set(mut data: Vec<u8>) -> Vec<Command> {
  let mut command_set = vec![];

  // Iterate over all commands
  loop {
    // Check if any commands are present
    if data.len() <= 2 {
      break;
    }
    if data[0] == 0 {
      break;
    }

    let command_length = data[0];
    let mut command = Command {
      length: i32::from(command_length),
      obj_id: i32::from(data[1]),
      id:     i32::from(data[2]),
      body:   vec![],
    };
    if command.length > 3 {
      command.body = data[3..].to_owned();
    }
    command_set.push(command);

    // Remove current command from the command set
    data = data[command_length as usize..].to_vec();
  }

  // Return the human-readable command set
  command_set
}
