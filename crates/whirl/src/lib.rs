// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#![feature(type_ascription, hash_set_entry, decl_macro, proc_macro_hygiene)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png",
  html_favicon_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
)]

#[macro_use] extern crate log;
// #[macro_use]
// extern crate simple_error;

#[cfg(windows)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod cli;

#[cfg(unix)] use signal_hook::consts::signal::{SIGINT, SIGTERM};
use whirl_config::Config;

pub struct Whirl;
impl Whirl {
  /// # Errors
  /// if the Logger fails to start.
  ///
  /// # Panics
  /// if the Logger fails to be created.
  pub async fn splash() -> Result<(), Box<dyn std::error::Error>> {
    // Environment
    std::env::set_var("DATABASE_URL", ".whirl/db.sqlite3");

    // Logging
    dotenv::dotenv().ok();
    human_panic::setup_panic!();
    if Config::get().whirlsplash.log.enable {
      let logger = flexi_logger::Logger::try_with_str(
        whirl_common::log::calculate_log_level(),
      )
      .unwrap();
      if std::env::var("LOG_FILE").unwrap_or_else(|_| "true".to_string())
        == "false"
        || !whirl_config::Config::get().whirlsplash.log.file
        || ({
          // Cheeky as all hell.
          let args = std::env::args().collect::<Vec<_>>();
          if args.len() == 2 { args[1] == "clean" } else { false }
        })
      {
        logger.start()?;
      } else {
        logger
          .print_message()
          .log_to_file(
            flexi_logger::FileSpec::default().directory(".whirl/log"),
          )
          .start()?;
      }
    }

    // Ctrl+C handling
    #[cfg(unix)]
    tokio::spawn(async move {
      for signal in signal_hook::iterator::Signals::new(&[SIGTERM, SIGINT])
        .unwrap()
        .forever()
      {
        info!("signal received: {:?}, killing whirl", signal);
        std::process::exit(0);
      }
    });

    crate::cli::Cli::execute().await;

    Ok(())
  }
}
