// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub worldsmaster_greeting: String,
  pub worldsmaster_username: String,
  pub distributor_port:      i32,
  pub hub_port:              i32,
}
impl Default for Config {
  fn default() -> Self {
    Config {
      worldsmaster_greeting: "Welcome to Whirlsplash!".to_string(),
      worldsmaster_username: "WORLDSMASTER".to_string(),
      distributor_port:      6650,
      hub_port:              5673,
    }
  }
}

pub fn get_config() -> Result<Config, confy::ConfyError> {
  let config: Config = confy::load_path("./.whirlrc.toml").unwrap();

  Ok(config)
}

pub fn store_config(config: Config) -> Result<(), confy::ConfyError> {
  confy::store_path("./.whirlrc.toml", config)
}
