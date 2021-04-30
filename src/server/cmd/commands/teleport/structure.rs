// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug)]
pub struct Teleport {
  pub room_id:    i8,
  pub exit_type:  u8,
  pub entry_type: u8,
  pub x:          i16,
  pub y:          i16,
  pub z:          i16,
  pub direction:  i16,
}

#[derive(Debug)]
pub struct TeleportLiteral {
  pub room_id:    i8,
  pub exit_type:  u8,
  pub entry_type: u8,
  pub x:          f32,
  pub y:          f32,
  pub z:          f32,
  pub direction:  f32,
}
