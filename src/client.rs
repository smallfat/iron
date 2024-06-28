use anyhow::Result;
use bytes::BytesMut;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::mpsc;
use crate::connection::{ConnContext, ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};

pub struct Client {
    context: ConnContext,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(&mut self, addr: T) -> Result<()> {
        let socket = TcpStream::connect(addr).await?;

        let (tx, rx) = mpsc::channel(4 * 1024);

        // set reader and writer
        let (read_half, write_half) = socket.into_split();

        let conn_reader = ConnReader {
            read_stream: read_half,
            read_buffer: [0; 1024],
        };

        let mut read_handler = ReadHandler {
            conn_reader,
        };

        tokio::spawn(async move {
            if let Ok(()) = read_handler.read().await {
                // graceful disconnection
            } else {
                // error
            }
        });

        let conn_writer = ConnWriter {
            write_stream: write_half,
            write_buffer: rx,
        };

        let write_handler = WriteHandler {
            conn_writer,
        };

        tokio::spawn(async move {
            if let Ok(()) = write_handler.write().await {
                // graceful disconnection
            } else {
                // error
            };
        });

        let addr = socket.peer_addr()?;

        self.context = ConnContext {peer_addr: addr, buffer: tx};

        Ok(())
    }

    pub fn sendData(data: BytesMut) {

    }
}