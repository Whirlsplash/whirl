// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

/// Sort a vector by alphabetical order based on the first character of each
/// string.
pub fn vec_alphabetically(vec: &mut [&str]) {
  vec.sort_by_key(|a| a.to_lowercase());
}
