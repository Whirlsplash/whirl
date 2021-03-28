use tokio::sync::mpsc;
use bytes::BytesMut;

pub mod auto;
mod cmd;
pub mod room;
mod parser;
mod peer;
mod shared;

type Tx = mpsc::UnboundedSender<BytesMut>;
type Rx = mpsc::UnboundedReceiver<BytesMut>;
