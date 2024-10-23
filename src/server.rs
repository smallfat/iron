use std::fmt::Debug;
use std::sync::Arc;
use anyhow::{Error, Result};
use tokio::net::{TcpListener, TcpStream};
use crate::common::{IMailData, Mail};
use crate::connection::{ConnContext, ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};
use crate::notifier::NotifierType;

pub struct Server<T> {
    addr: String,
    context: Option<ConnContext<T>>,
}

impl<T: IMailData + Send + Sync + Debug + 'static + Default> Server<T> {
    fn new(bind_addr: String) -> Server<T> {
        Server::<T> { addr: bind_addr, context: None }
    }

    pub async fn run(&mut self, notifier: NotifierType<T>) -> Result<()> {
        println!("server run");

        loop {
            println!("prepare for accept");
            if let Ok(socket) = self.accept().await {
                let addr = (&socket).peer_addr().unwrap();
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
                    if let Err(e) = read_handler.read().await {
                        println!("notifier2.OnError {}", e);
                        // error
                        notifier2.OnError(String::from("read data error"));
                    } else {
                        println!("notifier2.OnDisconnect");
                        // graceful disconnection
                        notifier2.OnDisconnect();
                    }
                });

                let mut conn_writer = ConnWriter {
                    write_stream: write_half,
                    write_buffer: rx,
                };

                let mut write_handler = WriteHandler::<T> {
                    conn_writer,
                };

                self.context = Some(ConnContext {peer_addr: addr, buffer: tx});

                let notifier3 = notifier.clone();
                _ = tokio::spawn(async move {
                    if let Ok(()) = write_handler.write().await {
                        println!("notifier3.OnDisconnect");
                        // graceful disconnection
                        notifier3.OnDisconnect();
                    } else {
                        // error
                        notifier3.OnError(String::from("write data error"));
                    };
                });
            } else {
                println!("accept error");
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

    pub async fn send_data(&self, data: Arc<Mail<T>>) -> Result<()> {
        let cxt = &self.context;
        match cxt {
            None => {
                return Err(anyhow::anyhow!("invalid connection context"));
            }
            Some(c) => {
                let buf = &c.clone().buffer;
                if let Err(err) = buf.send(data).await {
                    return Err(anyhow::Error::from(err));
                }
            }
        }

        Ok(())
    }
}

pub async fn start_server<T: IMailData + Send + Sync + Debug + Default + 'static>(addr: String, notifier: NotifierType<T>) -> Result<()> {
    let mut server = Server::new(addr);
    server.run(notifier).await
}