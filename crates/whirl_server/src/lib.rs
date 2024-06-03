// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Exposes the Distributor and Hub for further use.

#![feature(type_ascription, hash_set_entry, decl_macro, proc_macro_hygiene)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png",
  html_favicon_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
)]
#![allow(non_local_definitions, dead_code)]

#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

mod cmd;
mod interaction;
mod net;

mod distributor;
mod hub;
mod packet_parser;

use {
  crate::interaction::shared::Shared,
  std::{error::Error, fmt, net::SocketAddr, sync::Arc},
  tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
  },
};

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
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[async_trait]
pub trait Server {
  async fn listen(
    address: &str,
    server_type: ServerType,
  ) -> Result<(), Box<dyn Error>> {
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

      debug!(
        "server of type {} accepted client at {}",
        server_type.to_string(),
        address
      );

      tokio::spawn(async move {
        if let Err(e) = Self::handle(state, stream, address, counter).await {
          error!("an error occurred: {}", e);
        }

        if std::env::var("EXIT_ON_CLIENT_DISCONNECT")
          .unwrap_or_else(|_| "false".to_string())
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

pub mod make {
  use {
    crate::{Server, ServerType},
    tokio::task::JoinHandle,
    whirl_config::Config,
  };

  /// Spawn and return a thread handle for a Distributor sub-server.
  ///
  /// # Panics
  /// - A panic may occur if the TCP server is unable to bind the specified
  ///   port.
  #[must_use]
  pub fn distributor() -> JoinHandle<()> {
    tokio::spawn(async move {
      crate::distributor::Distributor::listen(
        &*format!(
          "{}:{}",
          Config::get().whirlsplash.ip,
          Config::get().distributor.port
        ),
        ServerType::Auto,
      )
      .await
      .unwrap();
    })
  }

  /// Spawn and return a thread handle for a Hub sub-server.
  ///
  /// # Panics
  /// - A panic may occur if the TCP server is unable to bind the specified
  ///   port.
  #[must_use]
  pub fn hub() -> JoinHandle<()> {
    tokio::spawn(async move {
      crate::hub::Hub::listen(
        &*format!(
          "{}:{}",
          Config::get().whirlsplash.ip,
          Config::get().hub.port
        ),
        ServerType::Room,
      )
      .await
      .unwrap();
    })
  }

  /// Spawn and return a vector of thread handles for each sub-server — which
  /// should be — instantiated by the `whirl_server` crate.
  ///
  /// # Panics
  /// - A panic may occur if the TCP server is unable to bind the specified
  ///   port.
  #[must_use]
  #[deprecated(note = "The `distributor` and `hub` functions are more \
                       extensible, use them instead.")]
  pub fn all() -> Vec<JoinHandle<()>> { vec![distributor(), hub()] }
}
