// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
  cmd::{constants::Command, extendable::Creatable},
  net::property_list::PropertyList,
};

pub struct SessionExit(pub PropertyList);
impl Creatable for SessionExit {
  fn create(&self) -> Vec<u8> {
    self.0.clone().as_bytes(Command::SessExit as i32, 0x01)
  }
}
