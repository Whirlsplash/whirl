// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use config::{ConfigError, File};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashConfig {
  pub worldsmaster_username: String,
  pub log_level:             i64,
  pub ip:                    String,
  pub prompt_ps1:            String,
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

  pub fn get() -> Config {
    return if let Err(why) = Self::load() {
      error!(
        "unable to load configuration file, reverting to default: {}",
        why
      );
      Self::default()
    } else {
      Self::load().unwrap()
    };
  }
}
impl Default for Config {
  fn default() -> Self {
    Config {
      whirlsplash: WhirlsplashConfig {
        worldsmaster_username: "WORLDSMASTER".to_string(),
        log_level:             1,
        ip:                    "0.0.0.0".to_string(),
        prompt_ps1:            "[WORLDSMASTER@Whirlsplash ~]$".to_string(),
      },
      distributor: DistributorConfig {
        worldsmaster_greeting: "Welcome to Whirlsplash!".to_string(),
        port:                  6650,
      },
      hub:         HubConfig {
        port: 5673
      },
    }
  }
}
