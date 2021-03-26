use tokio_util::codec::{Framed, BytesCodec};
use tokio::net::TcpStream;
use crate::server::Rx;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::server::shared::Shared;
use tokio::sync::mpsc;

pub struct Peer {
	pub bytes: Framed<TcpStream, BytesCodec>,
	// pub(crate) rx: Rx,
	pub rx: Rx,
}
impl Peer {
	pub async fn new(
		state: Arc<Mutex<Shared>>,
		bytes: Framed<TcpStream, BytesCodec>,
		username: String,
	) -> std::io::Result<Peer> {
		// let address = bytes.get_ref().peer_addr()?;
		let (tx, rx) = mpsc::unbounded_channel();
		// state.lock().await.peers.insert(address, tx);
		state.lock().await.peers.insert(username, tx);
		Ok(Peer { bytes, rx })
	}
}
