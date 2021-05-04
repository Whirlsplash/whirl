// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod routes;

pub struct Api;
impl Api {
  pub fn listen() {
    let _ = rocket::ignite()
      .mount("/", routes![routes::index])
      .mount("/api/v1", routes![routes::stats::statistics])
      .launch();
  }
}
