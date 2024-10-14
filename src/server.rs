use std::fmt::Debug;
use anyhow::{Error, Result};
use tokio::net::{TcpListener, TcpStream};
use crate::common::IMailData;
use crate::connection::{ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};
use crate::notifier::NotifierType;

pub struct Server {
    addr: String,
}

impl Server {
    fn new(bind_addr: String) -> Server {
        Server { addr: bind_addr }
    }

    pub async fn run<T: IMailData + Send + Sync + Debug + 'static>(&self, notifier: NotifierType<T>) -> Result<()> {
        println!("server run");

        loop {
            println!("prepare for accept");
            if let Ok(socket) = self.accept().await {
                println!("receive connection");

                let (read_half, write_half) = socket.into_split();
                let (tx, rx) = tokio::sync::mpsc::channel(4 * 1024);


                let conn_reader = ConnReader {
                    read_stream: read_half,
                    read_buffer: [0; 1024],
                };

                let notifier1 = notifier.clone();
                let mut read_handler = ReadHandler {
                    conn_reader,
                    notifier: notifier1,
                };

                let notifier2 = notifier.clone();
                let _ = tokio::spawn(async move {
                    if let Ok(()) = read_handler.read().await {
                        // graceful disconnection
                        notifier2.OnDisconnect();
                    } else {
                        // error
                        notifier2.OnError(String::from("read data error"));
                    }
                });

                let mut conn_writer = ConnWriter {
                    write_stream: write_half,
                    write_buffer: rx,
                };

                let mut write_handler = WriteHandler::<T> {
                    conn_writer,
                };

                let notifier3 = notifier.clone();
                _ = tokio::spawn(async move {
                    if let Ok(()) = write_handler.write().await {
                        // graceful disconnection
                        notifier3.OnDisconnect();
                    } else {
                        // error
                        notifier3.OnError(String::from("write data error"));
                    };
                });
            } else {
                // accept error
                notifier.OnError(String::from("accept error"));
                break;
            }
        }

        Ok(())
    }

    pub async fn accept(&self) -> Result<TcpStream> {
        if let Ok(listener) = TcpListener::bind(&self.addr).await {
            return match listener.accept().await {
                Ok((socket, _)) => {
                    println!("accept connection request");

                    Ok(socket)
                },
                Err(err) => {
                    println!("accept connection request failed");
                    Err(err.into())
                }
            }
        } else {
            Err(Error::msg("bind address error"))
        }


    }
}

pub async fn start_server<T: IMailData + Send + Sync + Debug + 'static>(addr: String, notifier: NotifierType<T>) -> Result<()> {
    let server = Server::new(addr);
    server.run::<T>(notifier).await
}