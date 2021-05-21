// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;

pub mod models;
mod schema;

use diesel::prelude::*;

// use crate::db::models::*;

pub fn establish_connection() -> SqliteConnection {
  let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "whirl.sqlite3".to_string());
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

/// Only works if you have a valid database already setup!
#[cfg(test)]
#[test]
#[ignore]
pub fn show_serials() {
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
