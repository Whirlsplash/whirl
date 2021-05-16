// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use config::{ConfigError, File};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashConfig {
  pub worldsmaster_username: String,
  pub ip:                    String,
  pub api:                   WhirlsplashApiConfig,
  pub prompt:                WhirlsplashPromptConfig,
  pub log:                   WhirlsplashLogConfig,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashApiConfig {
  pub port: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashPromptConfig {
  pub enable: bool,
  pub ps1:    String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashLogConfig {
  pub enable:     bool,
  pub level:      i64,
  pub everything: bool,
  pub test:       bool,
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
  pub fn refresh() { let _ = config::Config::new().refresh(); }

  fn load() -> Result<Self, ConfigError> {
    let mut s = config::Config::new();

    s.merge(File::with_name("./Whirl.toml").required(false))?;
    s.try_into()
  }

  pub fn get() -> Config {
    return if let Err(why) = Self::load() {
      error!(
        "unable to load configuration file, reverting to default value: {}",
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
        ip:                    "0.0.0.0".to_string(),
        api:                   WhirlsplashApiConfig {
          port: 80
        },
        prompt:                WhirlsplashPromptConfig {
          enable: false,
          ps1:    "[WORLDSMASTER@Whirlsplash ~]$".to_string(),
        },
        log:                   WhirlsplashLogConfig {
          enable:     true,
          level:      1,
          everything: false,
          test:       false,
        },
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
