// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::{io::Write, str::FromStr};

use sysinfo::SystemExt;

use crate::config::Config;

const FILES: [&str; 2] = ["README.rst", "Whirl.toml"];
const HELPABLES_BUILTINS: [&str; 8] = [
  "cat     - display the contents of a present file",
  "config  - manipulate the configuration",
  "echo    - display a line of predefined text",
  "exit    - end the process",
  "fetch   - a neofetch like utility loosely based on rfetch",
  "help    - you are here",
  "history - display the command history",
  "ls      - display the present files",
];
const HELPABLES_BUILTIN_CONFIG: [&str; 3] = [
  "help    - you are here",
  "refresh - reload the configuration file",
  "show    - display the current configuration",
];

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

pub fn builtin_echo(args: &[String]) -> i32 {
  println!("{}", args.join(" "));
  0
}

pub fn builtin_history(history: Vec<String>) -> i32 {
  for (index, cmd) in history.iter().enumerate() {
    println!("{}  {}", index, cmd.trim());
  }
  0
}

pub fn builtin_help() -> i32 {
  for help in HELPABLES_BUILTINS.iter() {
    println!("{}", help);
  }

  0
}

pub fn builtin_ls() -> i32 {
  for file in &FILES {
    print!("{}  ", file);
  }
  println!();

  0
}

pub async fn builtin_cat(args: &[String]) -> i32 {
  let file;
  if let Some(file_name) = args.get(0) {
    file = file_name.to_string();
  } else {
    return 0;
  };

  match file.as_str() {
    "README.rst" => {
      let mut easy = curl::easy::Easy::new();

      easy
        .url("https://raw.githubusercontent.com/Whirlsplash/whirl/develop/README.rst")
        .unwrap();

      let mut transfer = easy.transfer();
      transfer
        .write_function(|data| {
          std::io::stdout().write_all(data).unwrap();
          Ok(data.len())
        })
        .unwrap();
      transfer.perform().unwrap();
    }
    "Whirl.toml" => {
      colour::red_ln!("NOTE: This is just a wrapper for `config show`.");
      println!("{:#?}", Config::get());
    }
    _ => println!("/cat: {}: no such file or directory", file),
  }

  0
}

pub fn builtin_config(args: &[String]) -> i32 {
  match args.get(0) {
    Some(sub) =>
      match sub.as_str() {
        "show" => println!("{:#?}", Config::get()),
        "help" | "--help" | "-h" =>
          for help in HELPABLES_BUILTIN_CONFIG.iter() {
            println!("{}", help);
          },
        "refresh" => Config::refresh(),
        _ => println!("invalid arguments provided"),
      },
    None => println!("invalid amount arguments provided"),
  }
  0
}

pub fn builtin_fetch() -> i32 {
  // rfetch: https://github.com/Mangeshrex/rfetch

  let mut sys = sysinfo::System::new();
  sys.refresh_processes();

  println!("               ");
  println!("      .-.      os    {}", env!("CARGO_PKG_NAME"));
  println!("      oo|      ker   {}", env!("CARGO_PKG_VERSION"));
  println!("     / '\\      sh    /wsh");
  println!("    (\\_;/)     up    null");
  println!("               ");

  0
}
