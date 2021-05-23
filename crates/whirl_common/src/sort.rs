// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

/// Sort a vector by alphabetical order based on the first character of each
/// string.
pub fn sort_vec_alphabetically(vec: &mut Vec<&str>) {
  vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}
