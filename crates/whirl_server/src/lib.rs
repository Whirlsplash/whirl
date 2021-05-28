// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Exposes the Distributor and Hub for further use.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)] // clippy::pedantic
#![recursion_limit = "128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;

mod cmd;
mod interaction;
mod net;

pub mod distributor;
pub mod hub;
mod packet_parser;
mod types;

use std::{error::Error, fmt, net::SocketAddr, sync::Arc};

use tokio::{
  net::{TcpListener, TcpStream},
  sync::Mutex,
};
use whirl_config::Config;

use crate::interaction::shared::Shared;

/// The type of server the `listen` method of the `Server` trait will
/// implemented for.
#[derive(Debug)]
pub enum ServerType {
  AnonRoom,
  AnonUser,
  Auto,
  Room,
  User,
}
// https://stackoverflow.com/a/32712140/14452787
impl fmt::Display for ServerType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

#[async_trait]
pub trait Server {
  async fn listen(address: &str, server_type: ServerType) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    let state = Arc::new(Mutex::new(Shared::new()));
    let mut counter = 0;

    info!(
      "server of type {} now listening at {}",
      server_type.to_string(),
      address
    );

    loop {
      let (stream, address) = listener.accept().await?;
      counter += 1;
      let state = Arc::clone(&state);

      debug!("accepted client at {}", address);

      tokio::spawn(async move {
        if let Err(e) = Self::handle(state, stream, address, counter).await {
          error!("an error occurred: {}", e);
        }

        if std::env::var("EXIT_ON_CLIENT_DISCONNECT").unwrap_or_else(|_| "false".to_string())
          == "true"
        {
          std::process::exit(0);
        }
      });
    }
  }

  async fn handle(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    _address: SocketAddr,
    count: usize,
  ) -> Result<(), Box<dyn Error>>;
}

/// # Panics
/// - A panic may occur if the TCP server is unable to bind the specified port.
#[must_use]
pub fn make() -> Vec<tokio::task::JoinHandle<()>> {
  vec![
    tokio::spawn(async move {
      crate::distributor::Distributor::listen(
        &*format!("0.0.0.0:{}", Config::get().distributor.port),
        ServerType::Auto,
      )
      .await
      .unwrap();
    }),
    tokio::spawn(async move {
      crate::hub::Hub::listen(
        &*format!("0.0.0.0:{}", Config::get().hub.port),
        ServerType::Room,
      )
      .await
      .unwrap();
    }),
  ]
}
