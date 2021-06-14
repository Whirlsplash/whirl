// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { whirl::Whirl::splash().await }
