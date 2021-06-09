// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Database utilities, to interact with the database.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;

// use crate::db::models::*;

/// Establish a connection to the `SQLite` database.
///
/// # Panics
/// - May panic if the database URL is inaccessible.
#[must_use]
pub fn establish_connection() -> SqliteConnection {
  let database_url =
    std::env::var("DATABASE_URL").unwrap_or_else(|_| ".whirl/db.sqlite3".to_string());
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

/// Only works if you have a valid database already setup!
#[cfg(test)]
#[test]
#[ignore]
fn show_serials() {
  use crate::{models::SerialNumber, schema::serial_numbers::dsl::*};

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
