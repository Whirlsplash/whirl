// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

pub enum BuiltIn {
  Echo,
  History,
  Exit,
  Null,
  Help,
  Ls,
  Cat,
  Config,
  Fetch,
}
impl FromStr for BuiltIn {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "echo" => Ok(Self::Echo),
      "history" => Ok(Self::History),
      "exit" => Ok(Self::Exit),
      "null" => Ok(Self::Null),
      "help" => Ok(Self::Help),
      "ls" => Ok(Self::Ls),
      "cat" => Ok(Self::Cat),
      "config" => Ok(Self::Config),
      "fetch" => Ok(Self::Fetch),
      _ => Err(()),
    }
  }
}
