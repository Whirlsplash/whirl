// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub struct Command {
  pub length: i32,
  pub obj_id: i32,
  pub id:     i32,
  pub body:   Vec<u8>,
}
impl Command {
  /// Create and return a new `Command` with default values (`0`s and empty).
  pub fn _new() -> Self { Self::default() }

  pub fn _from_byte(mut data: Vec<u8>) -> Vec<Self> {
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
      let mut command = Self {
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
}
impl Default for Command {
  fn default() -> Self {
    Self {
      length: 0,
      obj_id: 0,
      id:     0,
      body:   vec![],
    }
  }
}
