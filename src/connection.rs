use std::net::SocketAddr;
use bytes::{BytesMut};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct ConnContext {
    pub peer_addr: SocketAddr,
}

pub struct ConnReader {
    pub read_stream: OwnedReadHalf,

    pub read_buffer: BytesMut,
}

pub struct ConnWriter {
    pub write_stream: OwnedWriteHalf,

    pub write_buffer: BytesMut,
}