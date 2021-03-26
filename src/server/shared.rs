use std::collections::HashMap;
// use std::net::SocketAddr;
use bytes::BytesMut;
use crate::server::Tx;

pub struct Shared {
	pub peers: HashMap<String, Tx>,
}
impl Shared {
	pub fn new() -> Self {
		Shared {
			peers: HashMap::new(),
		}
	}

	pub async fn broadcast(&mut self, /* sender: &str, */ message: &[u8]) {
		// debug!("peer sent message: {:?}", message);
		// debug!("peer count: {}", self.peers.len());
		// debug!("peers: {:?}", self.peers);
		for peer in self.peers.iter_mut() {
			// debug!("peer: {:?}", peer);
			// TODO:
			//  thread 'tokio-runtime-worker' panicked at 'called `Option::unwrap()` on a `None` value'
			peer.1.send(BytesMut::from(message)).unwrap();
		}
	}

	// pub async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
	// 	for peer in self.peers.iter_mut() {
	// 		if *peer.0 != sender {
	// 			let _ = peer.1.send(message.into());
	// 		}
	// 	}
	// }
}
