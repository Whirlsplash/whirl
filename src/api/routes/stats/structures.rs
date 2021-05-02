// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Statistics {
  pub system:  StatisticsSystem,
  pub process: StatisticsProcess,
}
#[derive(Serialize)]
pub struct StatisticsSystem {
  #[serde(rename = "type")]
  pub os_type: String,
  pub release: String,
  pub uptime:  String,
}
#[derive(Serialize)]
pub struct StatisticsProcess {
  pub memory_usage: String,
  pub cpu_usage:    String,
  // pub uptime:       String,
}
