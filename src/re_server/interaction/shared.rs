use std::collections::HashMap;

use bytes::BytesMut;

use crate::re_server::types::Tx;

pub struct Shared {
  pub peers: HashMap<String, Tx>,
}
impl Shared {
  pub fn new() -> Self {
    Shared {
      peers: HashMap::new(),
    }
  }

  pub async fn broadcast(&mut self, message: &[u8]) {
    for peer in self.peers.iter_mut() {
      peer.1.send(BytesMut::from(message)).unwrap();
    }
  }
}
impl Default for Shared {
  fn default() -> Self { Self::new() }
}
