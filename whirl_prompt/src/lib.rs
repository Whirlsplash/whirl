// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod builtins;
mod structure;

use std::{io, io::Write, str::FromStr};

use whirl_config::Config;

use crate::{
  builtins::{
    builtin_cat,
    builtin_config,
    builtin_echo,
    builtin_fetch,
    builtin_help,
    builtin_history,
    builtin_ls,
    BuiltIn,
  },
  structure::Command,
};

pub struct Prompt {
  history: Vec<String>,
}
impl Prompt {
  pub async fn handle() -> ! {
    let mut prompt = Prompt {
      history: vec![]
    };

    loop {
      Prompt::write_prompt();
      Prompt::process_command(
        Prompt::tokenize_command(prompt.read_command()),
        prompt.history.clone(),
      )
      .await;
    }
  }

  fn write_prompt() {
    print!("{} ", Config::get().whirlsplash.prompt.ps1);
    io::stdout().flush().unwrap();
  }

  fn read_command(&mut self) -> String {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("failed to read command from stdin");

    if input.len() > 2 {
      self.history.push(input.clone());
    } else {
      input = "null".to_string();
    }

    input
  }

  fn tokenize_command(c: String) -> Command {
    let mut command_split: Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();

    Command {
      keyword: command_split.remove(0),
      args:    command_split,
    }
  }

  // TODO: Find a way to make this access itself `history` doesn't have to be
  // passed everytime.
  async fn process_command(c: Command, history: Vec<String>) -> i32 {
    match BuiltIn::from_str(&c.keyword) {
      Ok(BuiltIn::Echo) => builtin_echo(&c.args),
      Ok(BuiltIn::Exit) => std::process::exit(0),
      Ok(BuiltIn::History) => builtin_history(history),
      Ok(BuiltIn::Null) => 0,
      Ok(BuiltIn::Help) => builtin_help(),
      Ok(BuiltIn::Ls) => builtin_ls(),
      Ok(BuiltIn::Cat) => builtin_cat(&c.args).await,
      Ok(BuiltIn::Config) => builtin_config(&c.args),
      Ok(BuiltIn::Fetch) => builtin_fetch(),
      _ => {
        println!("wsh: command not found: {}", &c.keyword);
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
