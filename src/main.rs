// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

#[macro_use]
extern crate log;

use std::error::Error;

use structopt::clap::Shell;
use whirl::{
  cli::cli,
  config::Config,
  server::{
    distributor::Distributor,
    hub::Hub,
    server::{
      Server,
      ServerType::{AutoServer, RoomServer},
    },
  },
};
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Setup CLI
  let matches = cli().get_matches();

  // Set logging level
  let mut log_level = "whirl=error,whirl=warn,whirl=trace".to_string();
  if matches.is_present("debug") || Config::get()?.whirlsplash.log_level >= 2 {
    log_level += ",whirl=debug";
  }
  if matches.is_present("trace") || Config::get()?.whirlsplash.log_level >= 3 {
    log_level += ",whirl=trace";
  }
  std::env::set_var("RUST_LOG", log_level);

  // Set database URL
  std::env::set_var("DATABASE_URL", "sqlite:whirl.db");

  // Setup logging
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  // Handle CLI command
  if matches.is_present("run") {
    run().await.unwrap();
  } else if let Some(cmd) = matches.subcommand_matches("config") {
    if cmd.is_present("show") {
      println!("{:#?}", Config::get()?);
    }
  } else if let Some(shell) = matches.subcommand_matches("completions") {
    if shell.is_present("powershell") {
      cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::PowerShell, ".");
    } else if shell.is_present("bash") {
      cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, ".");
    } else if shell.is_present("elvish") {
      cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Elvish, ".");
    } else if shell.is_present("zsh") {
      cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, ".");
    } else if shell.is_present("fish") {
      cli().gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, ".");
    }
    debug!("generated shell completions");
  }

  Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
  let threads = vec![
    tokio::spawn(async move {
      let _ = Distributor::listen(
        &*format!("0.0.0.0:{}", Config::get().unwrap().distributor.port),
        AutoServer,
      )
      .await;
    }),
    tokio::spawn(async move {
      let _ = Hub::listen(
        &*format!("0.0.0.0:{}", Config::get().unwrap().hub.port),
        RoomServer,
      )
      .await;
    }),
  ];
  for thread in threads {
    let _ = thread.await;
  }

  Ok(())
}
