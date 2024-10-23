use std::fmt::Debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use bytes::BufMut;
use iron::server;
use iron::client::Client;
use iron::common::{IMailData, Mail};
use iron::notifier::{Notifier, NotifierType};

#[derive(Debug, Default)]
pub struct MessageType1 {
    size: u32,
    class: u8,
    data: Vec<u8>,
}


pub struct MyNotifier {

}


impl<T: Debug> Notifier<T> for MyNotifier {
    fn OnMsg(&self, msg: Arc<Mail<T>>) {
        println!("on message {:?}", msg);
    }
    fn OnConnect(&self) {
        println!("on connect");
    }
    fn OnDisconnect(&self) {
        println!("on disconnect");
    }
    fn OnError(&self, err: String) {
        println!("on error: {} ", err);
    }
}


impl IMailData for MessageType1 {
    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn set_data(&mut self, d: &[u8]) -> bool {
        self.data.clear();
        self.data.extend_from_slice(d);
        true
    }

    fn get_data_len(&self) -> u32 {
        u32::try_from(self.data.len())
            .map(|len| len)
            .unwrap()
    }
}

#[tokio::test(flavor="multi_thread", worker_threads=15)]
async fn test_build_connection() {
    let mut r = 0;

    let nt_server: NotifierType<MessageType1> = Arc::new(MyNotifier{});

    // start server
    tokio::spawn( async move {
        let _r = server::start_server::<MessageType1>(String::from("127.0.0.1:1821"), nt_server).await;
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let nt_client: NotifierType<MessageType1> = Arc::new(MyNotifier{});

    // start client
    if let Ok(client) = Client::<MessageType1>::connect("127.0.0.1:1821", nt_client).await {
        r = 0;
    } else {
        r = 1;
    }

    sleep(Duration::from_secs(10));
    assert_eq!(r, 0);
}



#[tokio::test(flavor="multi_thread", worker_threads=15)]
async fn test_send_data() {
    // start server
    let nt_server: NotifierType<MessageType1> = Arc::new(MyNotifier{});
    tokio::spawn( async move {
        let r = server::start_server::<MessageType1>(String::from("127.0.0.1:1821"), nt_server).await;
        if r.is_err() { assert!(false) }
    });

    tokio::time::sleep(Duration::from_secs(2)).await;

    let nt_client:NotifierType<MessageType1> = Arc::new(MyNotifier{});

    // start client
    if let Ok(mut client) = Client::connect("127.0.0.1:1821", nt_client).await {
        let mut mail = Box::new(MessageType1 { size: 10, class: 1, data: Vec::new()});
        mail.data.put_bytes(100, 1000);
        if let Ok(()) = client.send_data(Arc::new(mail)).await {
            println!("sent data");
            tokio::time::sleep(Duration::from_secs(5)).await;
            assert!(true)
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
}