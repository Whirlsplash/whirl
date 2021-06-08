// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use structopt::clap::{App, AppSettings, Arg, SubCommand};
use whirl_config::Config;

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

    match matches.subcommand() {
      ("run", _) => Self::run().await,
      ("config", Some(s_matches)) =>
        match s_matches.subcommand() {
          ("show", _) => println!("{:#?}", Config::get()),
          _ => unreachable!(),
        },
      ("clean", _) => {
        let cleanable_directories = vec!["log/"];
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
        SubCommand::with_name("run").about("Start the WorldServer"),
        SubCommand::with_name("config")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .subcommands(vec![SubCommand::with_name("show")]),
        SubCommand::with_name("clean")
          .about("Delete Whirl-generated files/ directories which are NOT critical. E.g., logs/"),
      ])
      .args(&[
        Arg::with_name("debug").short("d").long("debug"),
        Arg::with_name("trace").short("t").long("trace"),
      ])
  }

  async fn run() {
    vec![whirl_api::make()].extend(whirl_server::make::all());

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
