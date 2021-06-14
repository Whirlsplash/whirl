// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub struct Command {
  pub keyword: String,
  pub args:    Vec<String>,
}
impl Command {
  pub fn to_line(&self) -> String { format!("{} {}", self.keyword, self.args.join(" ")) }
}
