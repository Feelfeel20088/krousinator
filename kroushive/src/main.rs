use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};

use common::{
    registry::{
        HandlerMeta, HandlerRegistry, HiveContext, HiveHandleable, HiveHandlerMeta,
        HiveHandlerRegistry, HiveProducer,
    },
    types::{KuvasMap, ResponseWaiters, SharedHiveContext},
};
use common::axum_register::temp::AxumRouteMeta;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, Stream, StreamExt,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use uuid::Uuid;

mod models;


use tokio::{time::{sleep, Duration}};
use std::io::{self, Write};

const ASCII_ART: &str = r#"
    )                         )                 
 ( /( (           (        ( /( (    )      (   
 )\()))(    (    ))\  (    )\()))\  /((    ))\  
((_)\(()\   )\  /((_) )\  ((_)\((_)(_))\  /((_) 
| |(_)((_) ((_)(_))( ((_) | |(_)(_)_)((_)(_))   
| / /| '_|/ _ \| || |(_-< | ' \ | |\ V / / -_)  
|_\_\|_|  \___/ \_,_|/__/ |_||_||_| \_/  \___|                              
"#;

const FLAME_COLORS: [&str; 4] = [
    "\x1b[34m", // Blue
    "\x1b[31m", // Red
    "\x1b[33m", // Orange (approximated with Yellow)
    "\x1b[97m", // Bright White
];


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    println!("\n\nINIT phase start...\n\n");

    let mut lines: usize = 0;
    let mut startLine: usize = 0;
    let mut startLineSet: bool = false;
    for (i, c) in ASCII_ART.chars().enumerate() {
        if c == '\n' as char {
            lines += 1;
            if !startLineSet {
                startLine = i + 1
            }
        } else if !(c == ' ' as char) && !startLineSet {
            startLineSet = true;
        }
    }

    let total_lines = lines;
    lines = 0;
    
    for (i, c) in ASCII_ART.chars().enumerate().skip(startLine) {
        if c == '\n' {
            lines += 1;
        }
        
        let color_index = (lines * FLAME_COLORS.len()) / total_lines;
        let color = FLAME_COLORS[color_index.min(FLAME_COLORS.len() - 1)];
    
        print!("{}{}", color, c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(1)).await;
    }
    

    println!("\x1b[0m\n");
    

    let kroushive_interface = Arc::new(Mutex::new(HiveContext {}));
    let mut reg = HiveHandlerRegistry::new();
    let mut r = Router::new();

    // register all handlers
    for route in inventory::iter::<AxumRouteMeta> {
        println!("registering route: {}", route.path);
        r = (route.register_fn)(r);
    }
    for handler in inventory::iter::<HiveHandlerMeta> {
        reg.register(handler.name, handler.constructor);
    }

    let arc_reg = Arc::new(reg);

    // state
    let mut map = Arc::new(Mutex::new(HashMap::new()));
    let response_waiters: ResponseWaiters = Arc::new(Mutex::new(HashMap::new()));

    let webserver = TcpListener::bind("0.0.0.0:8080").await?;
    tokio::spawn(async move {
        
        axum::serve(webserver, r); // start server w routers
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
            Arc::clone(&arc_reg),
            Arc::clone(&kroushive_interface),
        ));
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: KuvasMap,
    response_waiters: ResponseWaiters,
    reg: Arc<HiveHandlerRegistry>,
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
                            println!("Found invalid JSON. Skipping.");
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

                    // switch mutex to tokio::sync::RwLock for better preformance
                    if reg.check(message_type) {
                        if let Some(req_id) = manual_request_id {
                            if let Some(waiter) = response_waiters.lock().await.remove(&req_id) {
                                let _ = waiter.send(json); // context handles model
                            } else {
                                println!("manual request ID not found in hashmap which should be there.")
                            }
                        } else {
                            if let Some(Ok(model)) = reg.get(message_type, &raw_text) {
                                model.handle(Arc::clone(&kroushive_interface)).await;
                            } else {
                                continue;
                            }
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
