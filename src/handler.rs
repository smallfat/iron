use anyhow::Result;
use std::net::Shutdown;
use crate::connection::{ConnReader, ConnWriter};

pub struct ReadHandler {
    pub conn_reader: ConnReader,
}

impl ReadHandler {
    pub async fn read(&self) -> Result<()> {
        Ok(())
    }
}

pub struct WriteHandler {
    pub conn_writer: ConnWriter,
}

impl WriteHandler {
    pub async fn write(&self) -> Result<()> {
        Ok(())
    }
}