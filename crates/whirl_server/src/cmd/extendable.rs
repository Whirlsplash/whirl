// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub trait Parsable {
  fn parse(data: Vec<u8>) -> Self;
}

pub trait Creatable {
  fn create(&self) -> Vec<u8>;
}

/// Having to do this makes me with there was operator overloading in Rust.
///
/// I *could* do this with a macro but since Text is the only struct that
/// implements this trait, it shouldn't be that big of a deal.
pub trait ParsableWithArguments {
  fn parse(data: Vec<u8>, args: &[&str]) -> Self;
}
