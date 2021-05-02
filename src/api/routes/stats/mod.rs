// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod structures;

use rocket_contrib::json::Json;

use crate::api::routes::stats::structures::Statistics;

#[get("/statistics")]
pub fn statistics() -> Json<Statistics> {
  Json(Statistics {
    message: "todo".to_string(),
  })
}
