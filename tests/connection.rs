use std::time::Duration;
use tokio::time::Sleep;
use iron::server;
use iron::client;
use iron::client::Client;

#[tokio::test]
async fn test_build_connection() {
    // start server
    tokio::spawn( async move {
        let r = server::start_server(String::from("127.0.0.1:1821")).await;
        if r.is_err() { assert!(false) }
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    // start client
    let c = Client{};
    if let Ok(r) = c.connect("127.0.0.1:1821").await {
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
    let c = Client{};
    if let Ok(r) = c.connect("127.0.0.1:1821").await {
        assert!(true)
    } else {
        assert!(false)
    }

    // send data


}