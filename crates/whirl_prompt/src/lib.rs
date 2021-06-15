// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The Whirl Shell, for local interaction.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
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

mod builtins;
mod structure;

use std::{io, io::Write, str::FromStr};

use whirl_config::Config;

use crate::{
  builtins::{
    builtin_cat,
    builtin_clear,
    builtin_config,
    builtin_echo,
    builtin_fetch,
    builtin_help,
    builtin_history,
    builtin_ls,
    structures::BuiltIn,
  },
  structure::Command,
};

pub struct Prompt {
  history: Vec<String>,
}
impl Prompt {
  /// Begin handling user input as the prompt.
  pub async fn handle() -> ! {
    let mut prompt = Self {
      history: vec![]
    };

    loop {
      Self::write_prompt();
      let command = Self::read_command();
      prompt.process_command(&Self::tokenize_command(&command));
    }
  }

  fn write_prompt() {
    print!("{} ", Config::get().whirlsplash.prompt.ps1);
    io::stdout().flush().unwrap();
  }

  fn read_command() -> String {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("failed to read command from stdin");

    if input.len() <= 2 {
      input = "null".to_string();
    }

    input
  }

  fn tokenize_command(c: &str) -> Command {
    let mut command_split: Vec<String> = c
      .split_whitespace()
      .map(std::string::ToString::to_string)
      .collect();

    Command {
      keyword: command_split.remove(0),
      args:    command_split,
    }
  }

  // TODO: Find a way to make this access itself `history` doesn't have to be
  // passed everytime.
  fn process_command(&mut self, c: &Command) -> i32 {
    let exit_code = match BuiltIn::from_str(&c.keyword) {
      Ok(BuiltIn::Echo) => builtin_echo(&c.args),
      Ok(BuiltIn::Exit) => std::process::exit(0),
      Ok(BuiltIn::History) => builtin_history(&self.history),
      Ok(BuiltIn::Null) => 0,
      Ok(BuiltIn::Help) => builtin_help(),
      Ok(BuiltIn::Ls) => builtin_ls(),
      Ok(BuiltIn::Cat) => builtin_cat(&c.args),
      Ok(BuiltIn::Config) => builtin_config(&c.args),
      Ok(BuiltIn::Fetch) => builtin_fetch(),
      Ok(BuiltIn::Clear) => builtin_clear(),
      _ => {
        println!("wsh: command not found: {}", &c.keyword);
        1
      }
    };

    if c.keyword != "null" {
      self.history.push(c.to_line());
    }

    exit_code
  }
}

#[cfg(test)]
mod tokenize_command {
  use crate::Prompt;

  #[test]
  #[ignore]
  fn empty_command() { assert_eq!("", Prompt::tokenize_command("").keyword) }

  #[test]
  fn test_keyword() { assert_eq!("test", Prompt::tokenize_command("test").keyword) }

  #[test]
  fn no_arg() { assert_eq!(0, Prompt::tokenize_command("test").args.len()) }

  #[test]
  fn one_arg() { assert_eq!(1, Prompt::tokenize_command("test one").args.len()) }

  #[test]
  fn multi_arg() { assert_eq!(3, Prompt::tokenize_command("test one two three").args.len()) }

  #[test]
  #[ignore]
  fn quotes() {
    assert_eq!(
      2,
      Prompt::tokenize_command("test \"one two\" three")
        .args
        .len()
    )
  }
}
