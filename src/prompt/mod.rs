// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod builtins;
mod structure;

use std::{io, io::Write, str::FromStr};

use crate::prompt::{
  builtins::{builtin_echo, BuiltIn},
  structure::Command,
};

pub struct Prompt;
impl Prompt {
  pub fn handle() {
    Prompt::write_prompt();
    Prompt::process_command(Prompt::tokenize_command(Prompt::read_command()));
  }

  fn write_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
  }

  fn read_command() -> String {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("failed to read command from stdin");

    input
  }

  fn tokenize_command(c: String) -> Command {
    let mut command_split: Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();

    Command {
      keyword: command_split.remove(0),
      args:    command_split,
    }
  }

  fn process_command(c: Command) -> i32 {
    match BuiltIn::from_str(&c.keyword) {
      Ok(BuiltIn::Echo) => builtin_echo(&c.args),
      Ok(BuiltIn::Exit) => std::process::exit(0),
      _ => {
        println!("{}: command not found", &c.keyword);
        1
      }
    }
  }
}

#[cfg(test)]
mod tokenize_command {
  use crate::prompt::Prompt;

  #[test]
  #[ignore]
  fn empty_command() { assert_eq!("", Prompt::tokenize_command("".to_string()).keyword) }

  #[test]
  fn test_keyword() { assert_eq!("test", Prompt::tokenize_command("test".to_string()).keyword) }

  #[test]
  fn no_arg() { assert_eq!(0, Prompt::tokenize_command("test".to_string()).args.len()) }

  #[test]
  fn one_arg() {
    assert_eq!(
      1,
      Prompt::tokenize_command("test one".to_string()).args.len()
    )
  }

  #[test]
  fn multi_arg() {
    assert_eq!(
      3,
      Prompt::tokenize_command("test one two three".to_string())
        .args
        .len()
    )
  }

  #[test]
  #[ignore]
  fn quotes() {
    assert_eq!(
      2,
      Prompt::tokenize_command("test \"one two\" three".to_string())
        .args
        .len()
    )
  }
}
