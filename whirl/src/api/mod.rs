// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use actix_web::web::resource;

mod routes;

pub struct Api;
impl Api {
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
