use bytes::BytesMut;
use tokio::sync::mpsc;

pub type Tx = mpsc::UnboundedSender<BytesMut>;
pub type Rx = mpsc::UnboundedReceiver<BytesMut>;
