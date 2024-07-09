use anyhow::{Error, Result};
use tokio::net::{TcpListener, TcpStream};
use crate::connection::{ConnReader, ConnWriter};
use crate::handler::{ReadHandler, WriteHandler};

pub struct Server {
    addr: String,
}

impl Server {
    fn new(bind_addr: String) -> Server {
        Server { addr: bind_addr }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            if let Ok(socket) = self.accept().await {
                let (read_half, write_half) = socket.into_split();
                let (tx, rx) = tokio::sync::mpsc::channel(4 * 1024);


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

                let mut conn_writer = ConnWriter {
                    write_stream: write_half,
                    write_buffer: rx,
                };

                let mut write_handler = WriteHandler {
                    conn_writer,
                };

                tokio::spawn(async move {
                    if let Ok(()) = write_handler.write().await {
                        // graceful disconnection
                    } else {
                        // error
                    };
                });
            } else {
                // accept error
            }
        }

        Ok(())
    }

    pub async fn accept(&self) -> Result<TcpStream> {
        if let Ok(listener) = TcpListener::bind(&self.addr).await {
            match listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    return Err(err.into());
                }
            }
        } else {
            Err(Error::msg("bind address error"))
        }


    }
}

pub async fn start_server(addr: String) -> Result<()> {
    let server = Server::new(addr);
    server.run().await
}