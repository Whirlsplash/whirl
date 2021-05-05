// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
  api::Api,
  config::Config,
  prompt::Prompt,
  server::{
    distributor::Distributor,
    hub::Hub,
    server::{
      Server,
      ServerType::{AutoServer, RoomServer},
    },
  },
};

pub async fn run() -> ! {
  let _threads = vec![
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

  std::thread::sleep(std::time::Duration::from_secs(2));
  loop {
    // TODO: Find a way to keep this persistent on the bottom row.
    Prompt::handle();
  }
}
