// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Assorted utilities, for global use.

#![feature(type_ascription, hash_set_entry, decl_macro, proc_macro_hygiene)]
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
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png",
  html_favicon_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
)]

pub mod log;
pub mod sort;
pub mod system;
