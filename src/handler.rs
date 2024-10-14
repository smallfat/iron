use std::fmt::Debug;
use std::io::Read;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::Arc;
use anyhow::Result;
use bytes::BufMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::common::{IMailData, Mail};
use crate::connection::{ConnReader, ConnWriter};
use crate::notifier::{Notifier, NotifierType};

pub struct ReadHandler<T> {
    pub conn_reader: ConnReader,
    pub notifier: NotifierType<T>,
}

impl<T: IMailData + Send + Sync + Debug + 'static + std::default::Default > ReadHandler<T> {
    pub async fn read(&mut self) -> Result<()> {

        // read data from half stream
        let mut data: Vec<u8> = Vec::with_capacity(10*1024);
        let mut packet_len: u32 = 0;
        while true {
            if let Ok(r) = self.conn_reader.read_stream.read(&mut self.conn_reader.read_buffer).await {
                if r != 0 {
                    data.extend_from_slice(&self.conn_reader.read_buffer[..r]);

                    if data.len() >= 4 {
                        packet_len = data.as_slice().read_u32().await.expect("fail to read packet length");
                    }

                    if packet_len > 0 {
                        let len_large = usize::try_from(packet_len)
                            .map(|pl| pl <= data.len())
                            .unwrap_or(false);

                        if len_large {
                            // all packet data received, deserialized to object
                            let t = crate::common::create::<T>();
                            t.set_data(&data[4..packet_len]);

                            // clear old packet data in vec
                        }
                    }

                    // let mut packet_slice = &self.conn_reader.read_buffer[..];
                    // if new_packet && r >= 4 {
                    //     packet_len = 0;
                    //
                    //     // got length
                    //     packet_size_array.copy_from_slice(&self.conn_reader.read_buffer[.. 4]);
                    // } else if new_packet && r<4 {
                    //     packet_size_array.copy_from_slice(&self.conn_reader.read_buffer[..r]);
                    //
                    // } else {
                    //     let (data_slice, _) = packet_slice.split_at(r);
                    //     data.put_slice(data_slice);
                    //
                    //     packet_len = packet_len + data_slice.len();
                    // }
                    //
                    // println!("recv: {:?} {:?}", r, &self.conn_reader.read_buffer);
                    // self.notifier.OnMsg();
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
        while let Some(mail) = self.conn_writer.write_buffer.recv().await {
            if let Err(err) = self.conn_writer.write_stream.write(mail.deref().get_data().as_slice()).await {
                println!("send mail error, {:?}", &err);
                return Err(anyhow::Error::from(err))
            } else {
                println!("sent mail {:?}", &mail);
            }
        }

        Ok(())
    }
}