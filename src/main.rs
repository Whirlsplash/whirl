// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl::cli::CLI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let matches = CLI::setup();

  // Setup logging
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  CLI::execute(matches).await;

  Ok(())
}
