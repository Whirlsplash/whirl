// Copyleft (ɔ) 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use crate::server::cmd::structure::Command;

/// Iterate over a command set in the from of bytes and return a list of
/// human-readable commands.
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
      length: command_length as i32,
      obj_id: data[1] as i32,
      id:     data[2] as i32,
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
