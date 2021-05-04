// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;

use crate::{
  api::Api,
  config::Config,
  server::{
    distributor::Distributor,
    hub::Hub,
    server::{
      Server,
      ServerType::{AutoServer, RoomServer},
    },
  },
};

pub async fn run() -> Result<(), Box<dyn Error>> {
  let threads = vec![
    tokio::spawn(async move {
      let _ = Distributor::listen(
        &*format!("0.0.0.0:{}", Config::get().unwrap().distributor.port),
        AutoServer,
      )
      .await;
    }),
    tokio::spawn(async move {
      let _ = Hub::listen(
        &*format!("0.0.0.0:{}", Config::get().unwrap().hub.port),
        RoomServer,
      )
      .await;
    }),
    tokio::spawn(async move {
      let _ = Api::listen();
    }),
  ];
  for thread in threads {
    let _ = thread.await;
  }

  Ok(())
}
