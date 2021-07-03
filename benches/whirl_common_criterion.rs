// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark_sort_vec_alphabetically(c: &mut Criterion) {
  c.bench_function("sort vec alphabetically", |b| {
    b.iter(|| whirl_common::sort::vec_alphabetically(&mut vec!["a", "c", "d", "b"]))
  });
}

fn criterion_benchmark_system_unixts_to_hrtime(c: &mut Criterion) {
  c.bench_function("system unix timestamp to human readable time", |b| {
    b.iter(|| whirl_common::system::unixts_to_hrtime(1623058677))
  });
}

criterion_group!(
  benches,
  criterion_benchmark_sort_vec_alphabetically,
  criterion_benchmark_system_unixts_to_hrtime,
);
criterion_main!(benches);
