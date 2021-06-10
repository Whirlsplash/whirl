// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

use structopt::clap::{App, AppSettings, Arg, SubCommand};
use whirl_config::Config;

enum RunType {
  Distributor,
  Hub,
  Api,
  All,
}
impl FromStr for RunType {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "distributor" => Ok(Self::Distributor),
      "hub" => Ok(Self::Hub),
      "api" => Ok(Self::Api),
      _ => Ok(Self::All),
    }
  }
}

pub struct Cli;
impl Cli {
  /// # Panics
  /// Though Rust thinks a panic might happen because of the `unreachable`
  /// macro, all CLI subcommands are handled so it is trully unreachable.
  pub async fn execute() {
    let matches = Self::cli().get_matches();

    if Config::get().whirlsplash.log.test {
      error!("error");
      warn!("warn");
      info!("info");
      debug!("debug");
      trace!("trace");
    }

    debug!("attempting to create .whirl directory...");
    match std::fs::create_dir(".whirl/") {
      Ok(_) => debug!("successfully created .whirl directory"),
      Err(e) => debug!("error creating .whirl directory: {}", e),
    }

    match matches.subcommand() {
      ("run", Some(s_matches)) =>
        Self::run({
          RunType::from_str(match s_matches.value_of("type") {
            Some("distributor") => "distributor",
            Some("hub") => "hub",
            Some("api") => "api",
            _ => "all",
          })
          .unwrap()
        })
        .await,
      ("config", Some(s_matches)) =>
        match s_matches.subcommand() {
          ("show", _) => println!("{:#?}", Config::get()),
          _ => unreachable!(),
        },
      ("clean", _) => {
        let cleanable_directories = vec![".whirl/log/"];
        for dir in cleanable_directories {
          let file_type = if dir.ends_with('/') {
            "directory"
          } else {
            "file"
          };
          info!("cleaning {}: {}", file_type, dir);
          if let Err(e) = std::fs::remove_dir_all(dir) {
            warn!("cannot delete {}: {}: {}", file_type, dir, e);
          }
        }
      }
      _ => unreachable!(),
    }
  }

  fn cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .settings(&[AppSettings::SubcommandRequiredElseHelp])
      .subcommands(vec![
        SubCommand::with_name("run")
          .about("Start the WorldServer or a single sub-server.")
          .arg(
            Arg::with_name("type")
              .required(false)
              .takes_value(true)
              .index(1)
              .possible_values(&["distributor", "hub", "api", "all"]),
          ),
        SubCommand::with_name("config")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .subcommands(vec![SubCommand::with_name("show")]),
        SubCommand::with_name("clean").about(
          "Delete Whirl-generated files/ directories which are NOT critical. E.g., .whirl/logs/",
        ),
      ])
      .args(&[
        Arg::with_name("debug").short("d").long("debug"),
        Arg::with_name("trace").short("t").long("trace"),
      ])
  }

  async fn run(server_type: RunType) {
    match server_type {
      RunType::Distributor => vec![whirl_server::make::distributor()],
      RunType::Hub => vec![whirl_server::make::hub()],
      RunType::Api => vec![whirl_api::make()],
      RunType::All =>
        vec![
          whirl_api::make(),
          whirl_server::make::distributor(),
          whirl_server::make::hub(),
        ],
    };

    if std::env::var("DISABLE_PROMPT").unwrap_or_else(|_| "false".to_string()) == "true"
      || !Config::get().whirlsplash.prompt.enable
    {
      info!("starting with prompt disabled");
      loop {
        std::thread::sleep(std::time::Duration::default());
      }
    }

    whirl_prompt::Prompt::handle().await;
  }
}
