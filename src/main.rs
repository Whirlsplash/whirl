// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Environment
  let matches = Cli::setup();

  // Logging
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  // Execution
  Cli::execute(matches).await;

  Ok(())
}
