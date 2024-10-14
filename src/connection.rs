use crate::common;
use std::net::{SocketAddr};
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use crate::common::{Mail};

pub struct ConnContext<T> {
    pub peer_addr: SocketAddr,
    pub buffer: mpsc::Sender<Arc<Mail<T>>>,
}

pub struct ConnReader {
    pub read_stream: OwnedReadHalf,

    pub read_buffer: [u8; 1024],
}

pub struct ConnWriter<T> {
    pub write_stream: OwnedWriteHalf,

    pub write_buffer: mpsc::Receiver<Arc<Mail<T>>>,
}

