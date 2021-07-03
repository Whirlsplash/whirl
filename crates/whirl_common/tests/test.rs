// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[test]
fn vec_alphabetically() {
  assert_eq!(vec!["a", "b", "c", "d"], {
    let mut vector = vec!["a", "c", "d", "b"];

    whirl_common::sort::vec_alphabetically(&mut vector);

    vector
  });
}

#[test]
fn seconds_to_hrtime() {
  assert_eq!(
    "125 weeks, 14 days, 9 hours, 37 mins, 57 secs",
    whirl_common::system::unixts_to_hrtime(1623058677),
  );
}
