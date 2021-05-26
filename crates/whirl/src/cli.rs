// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use structopt::clap::{App, AppSettings, Arg, SubCommand};
use whirl_config::Config;

use crate::subs::run;

pub struct Cli;
impl Cli {
  pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Self::cli().get_matches();

    if Config::get().whirlsplash.log.test {
      error!("error");
      warn!("warn");
      info!("info");
      debug!("debug");
      trace!("trace");
    }

    match matches.subcommand() {
      ("run", _) => run().await,
      ("config", Some(s_matches)) =>
        match s_matches.subcommand() {
          ("show", _) => println!("{:#?}", Config::get()),
          _ => unreachable!(),
        },
      ("clean", _) => {
        let cleanable_directories = vec!["log/"];
        for dir in cleanable_directories {
          let mut file_type = "directory";
          if !dir.ends_with('/') {
            file_type = "file";
          }
          info!("cleaning {}: {}", file_type, dir);
          if let Err(e) = std::fs::remove_dir_all(dir) {
            warn!("cannot delete {}: {}: {}", file_type, dir, e);
          }
        }
      }
      _ => unreachable!(),
    }

    Ok(())
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
}
