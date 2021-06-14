// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use bytes::BytesMut;

pub struct Shared {
  pub peers: HashMap<String, tokio::sync::mpsc::UnboundedSender<BytesMut>>,
}
impl Shared {
  pub fn new() -> Self {
    Self {
      peers: HashMap::new(),
    }
  }

  pub async fn broadcast(&mut self, message: &[u8]) {
    for peer in &mut self.peers {
      peer.1.send(BytesMut::from(message)).unwrap();
    }
  }
}
impl Default for Shared {
  fn default() -> Self { Self::new() }
}
