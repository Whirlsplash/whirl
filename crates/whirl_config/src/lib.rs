// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Configuration utilities, to interact with the configuration system.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod structures;

use config::{ConfigError, File};

use crate::structures::{
  DistributorConfig,
  HubConfig,
  WhirlsplashApiConfig,
  WhirlsplashConfig,
  WhirlsplashLogConfig,
  WhirlsplashPromptConfig,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub whirlsplash: WhirlsplashConfig,
  pub distributor: DistributorConfig,
  pub hub:         HubConfig,
}
impl Config {
  /// Re-fetch the configuration from the configuration file.
  #[deprecated(
    note = "the current implementation of the configurations system automatically performs \
            refreshes, this method has no effects"
  )]
  pub fn refresh() { let _ = config::Config::new().refresh(); }

  fn load() -> Result<Self, ConfigError> {
    let mut s = config::Config::new();

    s.merge(File::with_name("./Whirl.toml").required(false))?;
    s.try_into()
  }

  /// Get a certain configuration key or group from the configuration file.
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
          file:       true,
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
