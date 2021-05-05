// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use structopt::clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};

use crate::{config::Config, subs::run};

pub struct Cli;
impl Cli {
  pub fn setup() -> ArgMatches<'static> {
    let matches = Self::cli().get_matches();

    Self::calc_log_level(&matches);
    std::env::set_var("DATABASE_URL", "whirl.sqlite3");

    matches
  }

  pub async fn execute(matches: ArgMatches<'_>) {
    if matches.is_present("run") {
      run().await;
    } else if let Some(cmd) = matches.subcommand_matches("config") {
      if cmd.is_present("show") {
        println!("{:#?}", Config::get().unwrap());
      }
    } else if let Some(shell) = matches.subcommand_matches("completions") {
      if shell.is_present("powershell") {
        Self::cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::PowerShell, ".");
      } else if shell.is_present("bash") {
        Self::cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, ".");
      } else if shell.is_present("elvish") {
        Self::cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Elvish, ".");
      } else if shell.is_present("zsh") {
        Self::cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, ".");
      } else if shell.is_present("fish") {
        Self::cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, ".");
      }
      debug!("generated shell completions");
    }
  }

  fn calc_log_level(matches: &ArgMatches<'_>) {
    let mut log_level = "whirl=error,whirl=warn,whirl=trace".to_string();
    if matches.is_present("debug") || Config::get().unwrap().whirlsplash.log_level >= 2 {
      log_level += ",whirl=debug";
    }
    if matches.is_present("trace") || Config::get().unwrap().whirlsplash.log_level >= 3 {
      log_level += ",whirl=trace";
    }
    std::env::set_var("RUST_LOG", log_level);
  }

  fn cli<'a, 'b>() -> App<'a, 'b> {
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
        SubCommand::with_name("completions")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .about("Generate shell completions")
          .subcommands(vec![
            SubCommand::with_name("powershell"),
            SubCommand::with_name("bash"),
            SubCommand::with_name("elvish"),
            SubCommand::with_name("zsh"),
            SubCommand::with_name("fish"),
          ]),
      ])
      .args(&[
        Arg::with_name("debug").short("d").long("debug"),
        Arg::with_name("trace").short("t").long("trace"),
      ])
  }
}
