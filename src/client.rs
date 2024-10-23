use std::fmt::Debug;
use std::sync::Arc;
use anyhow::{Result};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::mpsc;
use crate::connection::{ConnContext, ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};
use crate::common::{IMailData, Mail};
use crate::notifier::NotifierType;

pub struct Client<T> {
    context: ConnContext<T>,
}

impl<T: IMailData + Send + Sync + Debug + 'static + Default> Client<T> {
    pub async fn connect<S: ToSocketAddrs>(addr: S, notifier: NotifierType<T>) -> Result<Self> {
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

        let n1 = notifier.clone();
        let n2 = notifier.clone();
        let mut read_handler = ReadHandler {
            conn_reader,
            notifier,
        };

        let _ = tokio::spawn(async move {
            if let Ok(()) = read_handler.read().await {
                println!("n1 OnDisconnect");
                // graceful disconnection
                n1.OnDisconnect();
            } else {
                println!("n1 OnError");
                // error
                n1.OnError(String::from("read data error"));
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
                n2.OnDisconnect();
            } else {
                // error
                n2.OnError(String::from("write data error"));
            };
        });

        Ok(Client{ context: ConnContext {peer_addr: addr, buffer: tx}})
    }

    pub async fn send_data(&mut self, data: Arc<Mail<T>>) -> Result<()> {
        if let Err(err) = self.context.buffer.send(data).await {
            return Err(anyhow::Error::from(err));
        }

        Ok(())
    }
}