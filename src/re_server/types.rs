// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use bytes::BytesMut;
use tokio::sync::mpsc;

pub type Tx = mpsc::UnboundedSender<BytesMut>;
pub type Rx = mpsc::UnboundedReceiver<BytesMut>;
