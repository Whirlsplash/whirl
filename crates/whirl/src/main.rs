// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use whirl::whirl::Whirl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { Whirl::splash().await }
