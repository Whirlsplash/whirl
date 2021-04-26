// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

#![feature(type_ascription, hash_set_entry, type_name_of_val)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

pub mod cli;
pub mod config;

pub mod db;
pub mod re_server;
pub mod utils;
