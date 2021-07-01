// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod structures;

use std::convert::TryFrom;

use actix_web::HttpResponse;
use num_traits::cast::AsPrimitive;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

use crate::routes::stats::structures::{Statistics, StatisticsProcess, StatisticsSystem};

// This is mostly for developmental testing, it consumes more CPU than it's
// worth.
pub fn statistics() -> HttpResponse {
  let mut sys = System::new_all();
  sys.refresh_all();

  let process = sys.process(get_current_pid().unwrap()).unwrap();

  HttpResponse::Ok().json(Statistics {
    system:  StatisticsSystem {
      os_type: sys.name().unwrap(),
      release: sys.kernel_version().unwrap(),
      uptime:  whirl_common::system::seconds_to_hrtime(usize::try_from(sys.uptime()).unwrap()),
    },
    process: StatisticsProcess {
      // (process.cpu_usage() * 100.0).round() / 100.0
      memory_usage: (process.memory() / 1000).to_string(),
      cpu_usage:    (process.cpu_usage() / sys.processors().len().as_(): f32).to_string(),
      // uptime: seconds_to_hrtime((sys.get_uptime() - process.start_time()) as usize),
    },
  })
}
