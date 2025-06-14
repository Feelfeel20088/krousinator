use axum::extract::State;
use axum::{routing::get, Router};
use futures_util::Stream;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{stream::{SplitStream, SplitSink}, SinkExt, StreamExt};
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;



type KuvasMap =  Arc<Mutex<HashMap<&'static str, tokio::sync::mpsc::UnboundedSender<Message>>>>;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // state
    let mut map = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new().route("/", get(|State(state): State<KuvasMap>| async move {
        let map = state.lock().await;
        for (key, tx) in map.iter() {
            println!("{}", key);
        }

    }).with_state(Arc::clone(&map)));
    
    
    let webserver = TcpListener::bind("0.0.0.0:8080").await?;
    tokio::spawn(async move {
        axum::serve(webserver, app).await.unwrap();
    });




    

    

    let websocket = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Let's spawn the handling of each connection in a separate task.
    loop {
        let (stream, addr) = websocket.accept().await.unwrap();
        tokio::spawn(handle_connection(stream, addr, Arc::clone(&map)));
    }

}



async fn handle_connection(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    clients: KuvasMap,
) {
    if let Ok(ws_stream) = accept_async(stream).await {
        let (mut write, mut read) = ws_stream.split();
        // let id = Uuid::new_v4();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
        let client_id = "222"; // or generate a UUID

        clients.lock().await.insert(client_id, tx);

        // Spawn task to forward messages from `rx` to the websocket
        let write_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if write.send(msg).await.is_err() {
                    break;
                }
            }
        });

        // Read loop (optional)
        while let Some(Ok(msg)) = read.next().await {
            println!("From {}: {}", client_id, msg.to_text().unwrap_or("[binary]"));
        }

        // Cleanup
        clients.lock().await.remove(&client_id);
        write_task.abort();
    }
}
