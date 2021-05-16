// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::{io::Write, str::FromStr};

use crate::config::Config;

const FILES: [&str; 2] = ["README.rst", "Whirl.toml"];

pub enum BuiltIn {
  Echo,
  History,
  Exit,
  Null,
  Help,
  Ls,
  Cat,
  Config,
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
  println!("echo    - display a line of predefined text");
  println!("history - display the command history");
  println!("exit    - end the process");
  println!("ls      - display the present files");
  println!("cat     - display the contents of a present file");
  println!("config  - manipulate the configuration");
  println!("help    - you are here");
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
        "help" | "--help" | "-h" => {
          println!("show    - display the current configuration");
          println!("help    - you are here");
          println!("refresh - reload the configuration file");
        }
        "refresh" => Config::refresh(),
        _ => println!("invalid arguments provided"),
      },
    None => println!("invalid amount arguments provided"),
  }
  0
}
