// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod structures;

use {
  crate::routes::stats::structures::{
    Statistics, StatisticsProcess, StatisticsSystem,
  },
  axum::response,
  std::convert::TryFrom,
  sysinfo::{get_current_pid, ProcessExt, System, SystemExt},
};

// This is mostly for developmental testing, it consumes more CPU than it's
// worth.
#[allow(clippy::unused_async)]
pub async fn statistics() -> impl response::IntoResponse {
  let mut sys = System::new_all();
  sys.refresh_all();

  let process = sys.process(get_current_pid().unwrap()).unwrap();

  (
    hyper::StatusCode::OK,
    response::Json(Statistics {
      system:  StatisticsSystem {
        os_type: sys.name().unwrap(),
        release: sys.kernel_version().unwrap(),
        uptime:  whirl_common::system::unixts_to_hrtime(
          usize::try_from(sys.uptime()).unwrap(),
        ),
      },
      process: StatisticsProcess {
        // (process.cpu_usage() * 100.0).round() / 100.0
        memory_usage: (process.memory() / 1000).to_string(),
        cpu_usage:    (process.cpu_usage() / sys.processors().len() as f32)
          .to_string(),
        // uptime: seconds_to_hrtime((sys.get_uptime() - process.start_time())
        // as usize),
      },
    }),
  )
}
