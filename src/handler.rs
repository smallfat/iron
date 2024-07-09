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
                if r != 0 {
                    println!("recv: {:?} {:?}",r, &self.conn_reader.read_buffer);
                }
            } else {
                println!("read data failed");
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
            println!("got mail and get ready to send, {:?}", &mail);
            if let Err(err) = self.conn_writer.write_stream.write(mail.deref().data.as_slice()).await {
                println!("send mail error, {:?}", &err);
                return Err(anyhow::Error::from(err))
            } else {
                println!("sent mail {:?}", &mail);
            }
        }

        Ok(())
    }
}