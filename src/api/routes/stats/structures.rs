// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Statistics {
  pub message: String,
}
