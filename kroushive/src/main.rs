use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post},
    Json, Router,
};

use common::{
    registry::{
        HandlerMeta, HandlerRegistry, HiveContext, HiveHandleable, HiveHandlerMeta,
        HiveHandlerRegistry, HiveProducer,
    },
    types::{KuvasMap, ResponseWaiters, SharedHiveContext},
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, Stream, StreamExt,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};


use once_cell::sync::Lazy;


use tokio_tungstenite::{accept_async, tungstenite::protocol::Message,};

use uuid::Uuid;

mod models;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let kroushive_interface = Arc::new(Mutex::new(HiveContext {}));
    let mut reg = Arc::new(Mutex::new(HiveHandlerRegistry::new()));

    let mut temp_reg = reg.lock().await;
    // regester all handles
    for handler in inventory::iter::<HiveHandlerMeta> {
        temp_reg.register(handler.name, handler.constructor);
    }

    

    // state
    let mut map = Arc::new(Mutex::new(HashMap::new()));
    let response_waiters: ResponseWaiters = Arc::new(Mutex::new(HashMap::new()));





    let webserver = TcpListener::bind("0.0.0.0:8080").await?;
    tokio::spawn(async move {
        // this is werid
        axum::serve(webserver, build_router()); // start server w routers
    });

    let websocket = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    loop {
        let (stream, addr) = websocket.accept().await.unwrap();
        tokio::spawn(handle_connection(
            stream,
            addr,
            // switch these to a one shot or store them into a globle object?
            Arc::clone(&map),
            Arc::clone(&response_waiters),
            Arc::clone(&reg),
            Arc::clone(&kroushive_interface),
        ));
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: KuvasMap,
    response_waiters: ResponseWaiters,
    reg: Arc<Mutex<HiveHandlerRegistry>>,
    kroushive_interface: SharedHiveContext,
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

                    println!("{}", raw_text); // debugging

                    let message_type = match json.get("_t").and_then(|v| v.as_str()) {
                        Some(t) => t,
                        None => {
                            println!("No '_t' field found in message. Skipping.");
                            continue;
                        }
                    };

                    let manual_request_id = json
                        .get("manual_request_id")
                        .and_then(|v| v.as_str())
                        .and_then(|s| Uuid::parse_str(s).ok());

                    match reg.lock().await.get(message_type, &raw_text) {
                        Some(model) => match model {
                            Ok(model) => {
                                if let Some(req_id) = manual_request_id {
                                    if let Some(waiter) =
                                        response_waiters.lock().await.remove(&req_id)
                                    {
                                        let _ = waiter.send(json);
                                    }
                                } else {
                                    model.handle(kroushive_interface.clone()).await
                                }
                            }
                            Err(_err) => continue,
                        },
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
