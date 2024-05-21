use anyhow::Result;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::connection::ConnContext;

pub struct Client {

}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(&self, addr: T) -> Result<ConnContext> {
        let socket = TcpStream::connect(addr).await?;

        let addr = socket.peer_addr()?;

        Ok(ConnContext {peer_addr: addr})
    }
}