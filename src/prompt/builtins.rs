// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

pub enum BuiltIn {
  Echo,
  History,
  Exit,
  Null,
  Help,
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
      _ => Err(()),
    }
  }
}

pub fn builtin_echo(args: &[String]) -> i32 {
  println!("{}", args.join(" "));
  0
}

pub fn builtin_history(history: Vec<String>) -> i32 {
  let mut index = 0;
  for cmd in &history {
    println!("{}  {}", index, cmd.trim());
    index += 1;
  }
  0
}

pub fn builtin_help() -> i32 {
  println!(
    "echo    - display a line of text\nhistory - manipulate the history list\nexit    - end the \
     application"
  );
  0
}
