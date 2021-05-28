// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

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
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WhirlsplashLogConfig {
  pub enable:     bool,
  pub level:      i64,
  pub everything: bool,
  pub test:       bool,
  pub file:       bool,
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
