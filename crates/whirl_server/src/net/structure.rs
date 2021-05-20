// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub struct NetworkProperty {
  pub prop_id: i32,
  pub value:   String,
}
impl NetworkProperty {
  pub fn new() -> Self { NetworkProperty::default() }
}
impl Default for NetworkProperty {
  fn default() -> Self {
    NetworkProperty {
      prop_id: 0,
      value:   "".to_string(),
    }
  }
}
