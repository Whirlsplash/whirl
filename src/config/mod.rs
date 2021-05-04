// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use config::{ConfigError, File};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashConfig {
  pub worldsmaster_username: String,
  pub log_level:             i64,
  pub ip:                    String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DistributorConfig {
  pub worldsmaster_greeting: String,
  pub port:                  i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HubConfig {
  pub port: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub whirlsplash: WhirlsplashConfig,
  pub distributor: DistributorConfig,
  pub hub:         HubConfig,
}
impl Config {
  fn load() -> Result<Self, ConfigError> {
    let mut s = config::Config::new();

    s.merge(File::with_name("./Whirl.toml").required(false))?;
    s.try_into()
  }

  pub fn get() -> Result<Self, ConfigError> { Self::load() }
}
