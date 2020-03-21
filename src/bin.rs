use futures_util::StreamExt;
use futures_util::sink::Sink;
use futures_util::SinkExt;
use std::pin::Pin;
use std::io::Error;
use tokio_tungstenite::accept_async;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn start() -> Result<(),Error> {
    let addr = "127.0.0.1:8080";

    let mut listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (mut write, read) = ws_stream.split();

    write.send(Message::text("test")).await.expect("failed to send message");

    let mut ws_stream = write.reunite(read).unwrap();

    ws_stream.close(None).await.expect("failed to close stream");

    //let ws_stream = 

    /*
    read.forward(write)
        .await
        .expect("Failed to forward message")*/
}

#[tokio::main]
pub async fn main() {
    if let Err(e) = start().await {
        println!("Error: {}", e);
    }
}
