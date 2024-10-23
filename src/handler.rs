use std::fmt::Debug;
use std::io::Read;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::Arc;
use anyhow::Result;
use bytes::BufMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::common::{GenericsFactory, IMailData, Mail};
use crate::connection::{ConnReader, ConnWriter};
use crate::notifier::{Notifier, NotifierType};

pub struct ReadHandler<T> {
    pub conn_reader: ConnReader,
    pub notifier: NotifierType<T>,
}

impl<T: IMailData + Send + Sync + Debug + 'static + Default> ReadHandler<T> {
    pub async fn read(&mut self) -> Result<()> {
        // read data from half stream
        let mut data: Vec<u8> = Vec::new();
        let mut packet_len: usize = 0;
        loop {
            // read header
            let tmp_len = self.conn_reader.read_stream.read_u32().await?;

            println!("read len: {}", tmp_len);
            packet_len = usize::try_from(tmp_len)
                .map(|pl| pl)
                .unwrap();
            data.resize(packet_len, 0);

            // read body
            if let Ok(r) = self.conn_reader.read_stream.read_exact( &mut data).await {
                if r == packet_len {
                    // new object
                    let mut obj = T::default();
                    obj.set_data(&data);

                    self.notifier.OnMsg(Arc::new(Mail::new(obj)));
                } else {
                    // error
                    println!("incorrect data len read");
                    return Err(anyhow::anyhow!("read data failed"));
                }
            } else {
                println!("read data failed");
                return Err(anyhow::anyhow!("read data failed"));
            }
        }

        Ok(())
    }
}

pub struct WriteHandler<T>
    where T: IMailData + Send + Sync + Debug {
    pub conn_writer: ConnWriter<T>,
}

impl<T: IMailData + Send + Sync + Debug> WriteHandler<T> {
    pub async fn write(&mut self) -> Result<()> {
        // write data to half stream
        loop {
            if let Some(mail) = self.conn_writer.write_buffer.recv().await {
                let m = mail.deref();

                if let Err(err) = self.conn_writer.write_stream.write(m.get_data_len().to_be_bytes().as_slice()).await {
                    println!("send mail len error, {:?}", &err);
                    return Err(anyhow::Error::from(err))
                } else {
                    println!("sent mail len: {:?}", &m.get_data_len());
                }

                if let Err(err) = self.conn_writer.write_stream.write(m.get_data().as_slice()).await {
                    println!("send mail error, {:?}", &err);
                    return Err(anyhow::Error::from(err))
                } else {
                    println!("sent mail {:?}", &m.get_data());
                }
            } else {
                break;
            }
        }

        Ok(())
    }
}