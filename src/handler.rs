use anyhow::Result;
use std::net::Shutdown;
use futures::future::ok;
use tokio::io::AsyncReadExt;
use crate::connection::{ConnReader, ConnWriter};

pub struct ReadHandler {
    pub conn_reader: ConnReader,
}

impl ReadHandler {
    pub async fn read(&mut self) -> Result<()> {

        // read data from half stream
        while 1 {
            if let Ok(r) = self.conn_reader.read_stream.read(&mut self.conn_reader.read_buffer).await {

            } else {
                break;
            }
        }

        Ok(())
    }
}

pub struct WriteHandler {
    pub conn_writer: ConnWriter,
}

impl WriteHandler {
    pub async fn write(&self) -> Result<()> {
        // write data to half stream

        Ok(())
    }
}