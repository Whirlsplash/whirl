// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub struct Command {
  pub length: i32,
  pub obj_id: i32,
  pub id:     i32,
  pub body:   Vec<u8>,
}
impl Command {
  pub fn _new() -> Self { Command::default() }
}
impl Default for Command {
  fn default() -> Self {
    Command {
      length: 0,
      obj_id: 0,
      id:     0,
      body:   vec![],
    }
  }
}
