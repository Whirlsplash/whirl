// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, Default)]
pub struct NetworkProperty {
  pub prop_id: i32,
  pub value:   String,
}
impl NetworkProperty {
  pub fn _new() -> Self { Self::default() }
}
