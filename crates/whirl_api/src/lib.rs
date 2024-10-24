// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The API, for external interaction.

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
#![allow(clippy::cast_precision_loss)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use axum::routing::get;

mod routes;

pub struct Api;
impl Api {
  /// Begin handling connections on the web-server.
  ///
  /// # Errors
  /// - An error may arise if the web-server is unable to bind the specified
  ///   port.
  ///
  /// # Panics
  /// - A panic may occur if the mpsc sender is unable to send a clone of the
  ///   server.
  pub async fn listen(address: &str) {
    // TODO: Version handler
    let app = axum::Router::new()
      .route("/", get(|| async { "Whirlsplash" }))
      .route("/api/v1/stats", get(routes::stats::statistics))
      .route("/api/v1/worlds/info", get(routes::worlds::info::info))
      .route("/api/v1/worlds/vip", get(routes::worlds::vip::vip));

    axum::Server::bind(&address.parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();

    info!("http api now listening at {}", address);
  }
}

/// Spawn and return a thread handle for the API — which
/// should be — instantiated by the `whirl_api` crate.
///
/// # Panics
/// - A panic may occur if the mpsc sender is unable to send a clone of the
///   server.
#[must_use]
pub fn make() -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    crate::Api::listen(&format!(
      "{}:{}",
      whirl_config::Config::get().whirlsplash.ip,
      whirl_config::Config::get().whirlsplash.api.port
    ))
    .await;
  })
}
