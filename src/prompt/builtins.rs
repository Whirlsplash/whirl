// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

pub enum BuiltIn {
  Echo,
  History,
  Exit,
}
impl FromStr for BuiltIn {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "echo" => Ok(BuiltIn::Echo),
      "history" => Ok(BuiltIn::History),
      "exit" => Ok(BuiltIn::Exit),
      _ => Err(()),
    }
  }
}

pub fn builtin_echo(args: &[String]) -> i32 {
  println!("{}", args.join(" "));
  0
}
