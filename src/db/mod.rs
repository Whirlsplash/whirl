// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod models;
mod schema;

use diesel::prelude::*;

// use crate::db::models::*;

pub fn establish_connection() -> SqliteConnection {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

/// Only works if you have a valid database already setup!
#[cfg(test)]
#[test]
#[ignore]
pub fn show_serials() {
  use crate::db::{models::SerialNumber, schema::serial_numbers::dsl::*};

  dotenv::dotenv().ok();

  let results = serial_numbers
    .limit(5)
    .load::<SerialNumber>(&establish_connection())
    .expect("error loading serial numbers table");

  println!("found {} results", results.len());
  for result in results {
    println!("{}", result.user_name);
  }
}
