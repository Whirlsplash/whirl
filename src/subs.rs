// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
  api::Api,
  config::Config,
  prompt::Prompt,
  server::{
    distributor::Distributor,
    hub::Hub,
    Server,
    ServerType::{AutoServer, RoomServer},
  },
};

pub async fn run() -> ! {
  let (tx, _rx) = std::sync::mpsc::channel();

  let _threads = vec![
    tokio::spawn(async move {
      let _ = Distributor::listen(
        &*format!("0.0.0.0:{}", Config::get().distributor.port),
        AutoServer,
      )
      .await;
    }),
    tokio::spawn(async move {
      let _ = Hub::listen(&*format!("0.0.0.0:{}", Config::get().hub.port), RoomServer).await;
    }),
    tokio::spawn(async move {
      let _ = Api::listen(
        tx,
        &*format!("0.0.0.0:{}", Config::get().whirlsplash.api.port),
      )
      .await;
    }),
  ];

  if std::env::var("DISABLE_PROMPT").unwrap_or_else(|_| "false".to_string()) == "true" {
    info!("starting with prompt disabled");
    loop {
      std::thread::sleep(std::time::Duration::default());
    }
  } else {
    std::thread::sleep(std::time::Duration::from_secs(2));
    Prompt::handle();
  }

  // actix_web::rt::System::new("").block_on(rx.recv().unwrap().stop(true));
}
