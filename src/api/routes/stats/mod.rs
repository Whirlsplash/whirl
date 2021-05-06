// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod structures;

use rocket_contrib::json::Json;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

use crate::{
  api::routes::stats::structures::{Statistics, StatisticsProcess, StatisticsSystem},
  utils::system::seconds_to_hrtime,
};

// This is mostly for developmental testing, it consumes more CPU than it's
// worth.
#[get("/statistics")]
pub fn statistics() -> Json<Statistics> {
  let mut sys = System::new_all();
  sys.refresh_all();

  let process = sys.get_process(get_current_pid().unwrap()).unwrap();

  Json(Statistics {
    system:  StatisticsSystem {
      os_type: sys.get_name().unwrap(),
      release: sys.get_kernel_version().unwrap(),
      uptime:  seconds_to_hrtime(sysinfo::System::new().get_uptime() as usize),
    },
    process: StatisticsProcess {
      memory_usage: (process.memory() / 1000).to_string(),
      // (process.cpu_usage() * 100.0).round() / 100.0
      cpu_usage:    (process.cpu_usage() / sys.get_processors().len() as f32).to_string(),
      // uptime:       seconds_to_hrtime((sys.get_uptime() - process.start_time()) as usize),
    },
  })
}