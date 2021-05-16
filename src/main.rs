// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl::{cli::Cli, config::Config, utils::log::calculate_log_level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Environment
  let matches = Cli::setup();

  // Logging
  dotenv::dotenv().ok();
  let logger = flexi_logger::Logger::with_str(calculate_log_level());
  if std::env::var("LOG_FILE").unwrap_or_else(|_| "true".to_string()) == "false"
    || !Config::get().whirlsplash.log.file
  {
    logger.start()?;
  } else {
    logger
      .print_message()
      .log_to_file()
      .directory("log")
      .start()?;
  }

  // Execution
  Cli::execute(matches).await.unwrap();

  Ok(())
}
