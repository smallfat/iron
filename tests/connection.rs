use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use bytes::BufMut;
use iron::server;
use iron::client::Client;
use iron::common::Mail;

#[tokio::test]
async fn test_build_connection() {
    // start server
    tokio::spawn( async move {
        let r = server::start_server(String::from("127.0.0.1:1821")).await;
        if r.is_err() { assert!(false) }
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    // start client
    if let Ok(client) = Client::connect("127.0.0.1:1821").await {
        assert!(true)
    } else {
        assert!(false)
    }

}

#[tokio::test]
async fn test_send_data() {
    // start server
    tokio::spawn( async move {
        let r = server::start_server(String::from("127.0.0.1:1821")).await;
        if r.is_err() { assert!(false) }
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    // start client
    if let Ok(mut client) = Client::connect("127.0.0.1:1821").await {
        let mut mail = Mail { data: Vec::new()};
        mail.data.put_bytes(b'i', 10);
        if let Ok(result) = client.send_data(Arc::new(Pin::new(&mut &mail))).await {
            assert!(true)
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }
}