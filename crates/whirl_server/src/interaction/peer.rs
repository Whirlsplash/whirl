// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use tokio::{
  net::TcpStream,
  sync::{mpsc, Mutex},
};
use tokio_util::codec::{BytesCodec, Framed};

use crate::interaction::shared::Shared;

pub struct Peer {
  pub bytes: Framed<TcpStream, BytesCodec>,
  pub rx:    mpsc::UnboundedReceiver<bytes::BytesMut>,
}
impl Peer {
  pub async fn new(
    state: Arc<Mutex<Shared>>,
    bytes: Framed<TcpStream, BytesCodec>,
    username: String,
  ) -> std::io::Result<Self> {
    let (tx, rx) = mpsc::unbounded_channel();
    state.lock().await.peers.insert(username, tx);

    Ok(Self {
      bytes,
      rx,
    })
  }

  pub async fn _change_username(
    self,
    state: Arc<Mutex<Shared>>,
    username: &str,
    new_username: &str,
  ) {
    // Remove peer from peers
    {
      state.lock().await.peers.remove(username);
    }

    // Add the peer back with the new username
    Self::new(state, self.bytes, new_username.to_string())
      .await
      .unwrap();
  }
}
