// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use whirl::whirl::Whirl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { Whirl::splash().await }
