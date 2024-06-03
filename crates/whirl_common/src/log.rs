// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use whirl_config::Config;

/// Grab the log level configuration key (`whirlsplash.log.level`) from the
/// configuration file and evaluate the proper log level.
#[must_use]
pub fn calculate_log_level() -> String {
  let mut level;

  level = match Config::get().whirlsplash.log.level {
    2 => "debug".to_string(),
    3 => "trace".to_string(),
    _ => "info".to_string(),
  };
  if !Config::get().whirlsplash.log.everything {
    level = format!("whirl={level}");
  }

  level
}
