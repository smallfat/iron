use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use bytes::BufMut;
use iron::server;
use iron::client::Client;
use iron::common::{IMailData, Mail};
use iron::notifier::{Notifier, NotifierType};

#[derive(Debug)]
pub struct MessageType1 {
    size: u32,
    class: u8,
    data: Vec<u8>,
}


pub struct MyNotifier {

}

impl<T> Notifier<T> for MyNotifier {
    fn OnMsg(&self, msg: Arc<Mail<MessageType1>>) {}
    fn OnConnect(&self) {}
    fn OnDisconnect(&self) {}
    fn OnError(&self, err: String) {}
}


impl IMailData for MessageType1 {
    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[tokio::test]
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



#[tokio::test(flavor="multi_thread", worker_threads=5)]
async fn test_send_data() {
    // start server
    let nt_server: NotifierType<MessageType1> = Arc::new(MyNotifier{});
    tokio::spawn( async move {
        let r = server::start_server::<MessageType1>(String::from("127.0.0.1:1821"), nt_server).await;
        if r.is_err() { assert!(false) }
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let nt_client:NotifierType<MessageType1> = Arc::new(MyNotifier{});

    // start client
    if let Ok(mut client) = Client::connect("127.0.0.1:1821", nt_client).await {
        let mut mail = Box::new(MessageType1 { size: 10, class: 1, data: Vec::new()});
        mail.data.put_bytes(1, 1000);
        if let Ok(()) = client.send_data(Arc::new(mail)).await {
            println!("sent data");
            sleep(Duration::from_secs(10));
            assert!(true)
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }
}