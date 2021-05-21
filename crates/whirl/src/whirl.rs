// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl_common::log::calculate_log_level;

use crate::cli::Cli;

pub struct Whirl;
impl Whirl {
  pub async fn splash() -> Result<(), Box<dyn Error>> {
    // Environment and CLI
    let matches = Cli::setup();

    // Logging
    dotenv::dotenv().ok();
    human_panic::setup_panic!();
    let logger = flexi_logger::Logger::with_str(calculate_log_level());
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

    Cli::execute(matches).await.unwrap();

    Ok(())
  }
}
