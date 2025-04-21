mod models;
use std::sync::Arc;

use models::*;

use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt}; // Import required extension traits
use tokio_tungstenite::tungstenite::Message;

use tokio::task;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel(32);
    // Establish connection
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            tx_clone.send().await.unwrap();
            
        }
    });

    let (mut ws_stream, _) = connect_async("wss://echo.websocket.events").await.expect("Failed to connect");

    println!("✅ Connected!");
    let (mut write, mut read) = ws_stream.split();
    let write_object = Arc::new(write);

    write_object.clone();

    while let Some(msg) = rx.recv().await {
        println!("📨 Received: {}", msg);
    }

    // 📨 Send a message to the server
    let msg = Message::Text("Hello from Rust client!".into());
    write.send(msg).await.unwrap();
    println!("📤 Sent message");

    // 📥 Read message from the server
    if let Some(Ok(msg)) = read.next().await {
        println!("📬 Received: {}", msg);
    }

    println!("👋 Done!");
}
