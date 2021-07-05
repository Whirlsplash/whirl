// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Serialize)]
pub struct Vip {
  pub vip:   bool,
  pub error: Option<String>,
}
