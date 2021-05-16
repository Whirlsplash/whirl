// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub fn sort_vec_alphabetically(vec: &mut Vec<&str>) {
  vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}
