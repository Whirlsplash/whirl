// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The API, for external interaction.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use actix_web::web::resource;

mod routes;

pub struct Api;
impl Api {
  /// Begin handling connections on the web-server.
  pub async fn listen(
    tx: std::sync::mpsc::Sender<actix_web::dev::Server>,
    address: &str,
  ) -> std::io::Result<()> {
    let mut sys = actix_web::rt::System::new("api");

    let server = actix_web::HttpServer::new(|| {
      actix_web::App::new()
        .wrap(actix_cors::Cors::default().allow_any_origin())
        .service(resource("/").to(|| async { "Whirlsplash" }))
        .service(resource("/api/v1/statistics").to(routes::stats::statistics))
    })
    .bind(address)?
    .run();

    info!("http api now listening at {}", address);

    let _ = tx.send(server.clone());

    sys.block_on(server)
  }
}

pub fn make() -> tokio::task::JoinHandle<()> {
  // actix_web::rt::System::new("").block_on(rx.recv().unwrap().stop(true));

  tokio::spawn(async move {
    let _ = crate::Api::listen(
      std::sync::mpsc::channel().0,
      &*format!(
        "0.0.0.0:{}",
        whirl_config::Config::get().whirlsplash.api.port
      ),
    )
    .await;
  })
}
