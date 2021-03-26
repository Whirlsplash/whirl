use tokio::sync::mpsc;
use bytes::BytesMut;

pub mod auto;
mod cmd;
pub mod room;
mod shared;
mod peer;

type Tx = mpsc::UnboundedSender<BytesMut>;
type Rx = mpsc::UnboundedReceiver<BytesMut>;
