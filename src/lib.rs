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

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;

pub mod cli;
pub mod config;
pub mod subs;

pub mod api;
pub mod db;
pub mod server;
pub mod utils;
