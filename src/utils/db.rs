// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn get_pool() -> Result<SqlitePool, Box<dyn Error>> {
  let pool = SqlitePoolOptions::new()
    .max_connections(20)
    .connect(&std::env::var("DATABASE_URL")?)
    .await?;

  debug!(
    "connected to database at url '{}'",
    &std::env::var("DATABASE_URL")?
  );

  Ok(pool)
}
