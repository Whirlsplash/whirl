// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl::{cli::Cli, utils::log::calculate_log_level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Environment
  let matches = Cli::setup();

  // Logging
  dotenv::dotenv().ok();
  flexi_logger::Logger::with_str(calculate_log_level())
    .log_to_file()
    .directory("log")
    .print_message()
    .start()?;

  // Execution
  Cli::execute(matches).await;

  Ok(())
}
