// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
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
      "echo" => Ok(BuiltIn::Echo),
      "history" => Ok(BuiltIn::History),
      "exit" => Ok(BuiltIn::Exit),
      "null" => Ok(BuiltIn::Null),
      "help" => Ok(BuiltIn::Help),
      "ls" => Ok(BuiltIn::Ls),
      "cat" => Ok(BuiltIn::Cat),
      "config" => Ok(BuiltIn::Config),
      "fetch" => Ok(BuiltIn::Fetch),
      _ => Err(()),
    }
  }
}
