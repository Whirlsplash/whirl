// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod constants;
pub mod structures;

use {
  constants::{
    FILES, HELPABLES_BUILTINS, HELPABLES_BUILTIN_CONFIG,
    HELPABLES_BUILTIN_FETCH,
  },
  std::io::Write,
  sysinfo::SystemExt,
  whirl_config::Config,
};

pub fn builtin_echo(args: &[String]) -> i32 {
  println!("{}", args.join(" "));
  0
}

pub fn builtin_history(history: &[String]) -> i32 {
  for (index, cmd) in history.iter().enumerate() {
    println!("{}  {}", index, cmd.trim());
  }
  0
}

pub fn builtin_help() -> i32 {
  for help in &HELPABLES_BUILTINS {
    println!("{help}");
  }

  0
}

pub fn builtin_ls() -> i32 {
  for file in &FILES {
    print!("{file}  ");
  }
  println!();

  0
}

pub fn builtin_cat(args: &[String]) -> i32 {
  let file;
  if let Some(file_name) = args.first() {
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
    "Config.toml" => {
      colour::red_ln!("NOTE: This is just a wrapper for `config show`.");
      println!("{:#?}", Config::get());
    }
    _ => println!("/cat: {file}: no such file or directory"),
  }

  0
}

pub fn builtin_config(args: &[String]) -> i32 {
  match args.first() {
    Some(sub) => match sub.as_str() {
      "show" => println!("{:#?}", Config::get()),
      "help" | "--help" | "-h" =>
        for help in &HELPABLES_BUILTIN_CONFIG {
          println!("{help}");
        },
      // "refresh" => Config::refresh(),
      _ => println!("invalid arguments provided"),
    },
    None => println!("invalid amount arguments provided"),
  }
  0
}

pub fn builtin_fetch(args: &[String]) -> i32 {
  // rfetch: https://github.com/Mangeshrex/rfetch

  let mut sys = sysinfo::System::new();
  sys.refresh_processes();
  let mut whirl_image = false;

  #[allow(clippy::single_match)]
  match args.first() {
    Some(sub) => match sub.as_str() {
      "--whirl" | "-w" => whirl_image = true,
      "help" | "--help" | "-h" => {
        for help in &HELPABLES_BUILTIN_FETCH {
          println!("{help}");
        }
        return 0;
      }
      _ => println!("invalid arguments provided"),
    },
    _ => (),
  }

  if whirl_image {
    println!(" -----:::::--- ");
    println!(" ---:///::/+:- os    {}", env!("CARGO_PKG_NAME"));
    println!(" ---//-//--//- ker   {}", env!("CARGO_PKG_VERSION"));
    println!(" -:/:+-+:://-- sh    /wsh");
    println!(" -//:+-::::--- up    null");
    println!(" --:::-------- ");
  } else {
    println!("               ");
    println!("      .-.      os    {}", env!("CARGO_PKG_NAME"));
    println!("      oo|      ker   {}", env!("CARGO_PKG_VERSION"));
    println!("     / '\\      sh    /wsh");
    println!("    (\\_;/)     up    null");
    println!("               ");
  }

  0
}

pub fn builtin_clear() -> i32 {
  println!("\x1B[2J\x1B[1;1H");

  0
}
