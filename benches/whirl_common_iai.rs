// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

fn iai_benchmark_sort_vec_alphabetically() {
  whirl_common::sort::vec_alphabetically(&mut vec!["a", "c", "d", "b"])
}

fn iai_benchmark_system_unixts_to_hrtime() -> String {
  whirl_common::system::unixts_to_hrtime(1623058677)
}

iai::main!(
  iai_benchmark_sort_vec_alphabetically,
  iai_benchmark_system_unixts_to_hrtime
);
