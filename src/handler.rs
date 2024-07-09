use std::ops::Deref;
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::connection::{ConnReader, ConnWriter};

pub struct ReadHandler {
    pub conn_reader: ConnReader,
}

impl ReadHandler {
    pub async fn read(&mut self) -> Result<()> {

        // read data from half stream
        while true {
            if let Ok(r) = self.conn_reader.read_stream.read(&mut self.conn_reader.read_buffer).await {
                println!("recv: {:?} {:?}",r, &self.conn_reader.read_buffer);
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
    pub async fn write(&mut self) -> Result<()> {
        // write data to half stream
        while let Some(mail) = self.conn_writer.write_buffer.recv().await {
            if let Err(err) = self.conn_writer.write_stream.write(mail.deref().data.as_slice()).await {
                return Err(anyhow::Error::from(err))
            }
        }

        Ok(())
    }
}