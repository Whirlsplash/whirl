// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub const FILES: [&str; 2] = ["README.rst", "Config.toml"];
pub const HELPABLES_BUILTINS: [&str; 9] = [
  "cat     - display the contents of a present file",
  "clear   - clear the display (standard out)",
  "config  - manipulate the configuration",
  "echo    - display a line of predefined text",
  "exit    - end the process",
  "fetch   - a neofetch like utility loosely based on rfetch",
  "help    - you are here",
  "history - display the command history",
  "ls      - display the present files",
];
pub const HELPABLES_BUILTIN_CONFIG: [&str; 2] = [
  "help    - you are here",
  // "refresh - reload the configuration file",
  "show    - display the current configuration",
];
