// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
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

#[macro_use]
extern crate log;
// #[macro_use]
// extern crate simple_error;

#[cfg(windows)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod cli;

use whirl_config::Config;

pub struct Whirl;
impl Whirl {
  /// # Errors
  /// - An error may arise if logger fails to start.
  pub async fn splash() -> Result<(), Box<dyn std::error::Error>> {
    // Environment
    std::env::set_var("DATABASE_URL", "whirl.sqlite3");

    // Logging
    dotenv::dotenv().ok();
    human_panic::setup_panic!();
    if Config::get().whirlsplash.log.enable {
      let logger = flexi_logger::Logger::with_str(whirl_common::log::calculate_log_level());
      if std::env::var("LOG_FILE").unwrap_or_else(|_| "true".to_string()) == "false"
        || !whirl_config::Config::get().whirlsplash.log.file
        || std::env::args().collect::<Vec<_>>()[1] == "clean"
      // Cheeky as all hell.
      {
        logger.start()?;
      } else {
        logger
          .print_message()
          .log_to_file()
          .directory("log")
          .start()?;
      }
    }

    crate::cli::Cli::execute().await;

    Ok(())
  }
}
