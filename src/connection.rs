use crate::common;
use std::net::{SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use crate::common::{Mail};

pub struct ConnContext {
    pub peer_addr: SocketAddr,
    pub buffer: mpsc::Sender<Arc<Pin<Mail>>>,
}

pub struct ConnReader {
    pub read_stream: OwnedReadHalf,

    pub read_buffer: [u8; 1024],
}

pub struct ConnWriter {
    pub write_stream: OwnedWriteHalf,

    pub write_buffer: mpsc::Receiver<Arc<Pin<common::Mail>>>,
}

