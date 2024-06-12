// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  crate::cmd::commands::register_object_id::RegisterObjectId, bytes::BytesMut,
  std::collections::HashMap,
};

pub struct Shared {
  pub peers:      HashMap<String, tokio::sync::mpsc::UnboundedSender<BytesMut>>,
  pub object_ids: Vec<RegisterObjectId>,
}
impl Shared {
  pub fn new() -> Self { Self { peers: HashMap::new(), object_ids: vec![] } }

  #[allow(clippy::unused_async)]
  pub async fn broadcast(&mut self, message: &[u8]) {
    for peer in &mut self.peers {
      peer.1.send(BytesMut::from(message)).unwrap();
    }
  }
}
impl Default for Shared {
  fn default() -> Self { Self::new() }
}
