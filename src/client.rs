use std::pin::Pin;
use std::sync::Arc;
use anyhow::Result;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::mpsc;
use crate::connection::{ConnContext, ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};
use crate::common::Mail;

pub struct Client {
    context: ConnContext,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Self> {
        let socket = TcpStream::connect(addr).await?;
        let addr = socket.peer_addr()?;

        println!("client, connect ok , peer addr {:?}", addr);

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

        let _ = tokio::spawn(async move {
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

        let mut write_handler = WriteHandler {
            conn_writer,
        };

        let _ = tokio::spawn(async move {
            if let Ok(()) = write_handler.write().await {
                // graceful disconnection
            } else {
                // error
            };
        });

        Ok(Client{ context: ConnContext {peer_addr: addr, buffer: tx}})
    }

    pub async fn send_data(&mut self, data: Arc<Mail>) -> Result<()> {
        if let Err(err) = self.context.buffer.send(data).await {
            return Err(anyhow::Error::from(err));
        }

        Ok(())
    }
}