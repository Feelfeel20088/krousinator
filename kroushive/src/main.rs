use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::Router;

use common::axum_register::temp::AxumRouteMeta;
use common::registry::context::KrousEnvelopeRecv;
use common::{
    registry::{HiveContext, HiveHandlerMeta, HiveHandlerRegistry},
    types::{KuvasMap, ResponseWaiters, SharedHiveContext},
};

use futures_util::{SinkExt, StreamExt};

use serde_json::Value;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use uuid::Uuid;

mod models;

use std::io::{self, Write};
use tokio::time::{sleep, Duration};

// constants
const WEBSERVER_URL: &str = "0.0.0.0:8080";
const TUNGSTENITE_URL: &str = "0.0.0.0:8080";
const DEBUG: bool = true;

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
    "\x1b[33m", // Orange
    "\x1b[97m", // Bright White
];

async fn banner() {
    let mut lines: usize = 0;
    let mut start_line: usize = 0;
    let mut start_line_set: bool = false;
    for (i, c) in ASCII_ART.chars().enumerate() {
        if c == '\n' as char {
            lines += 1;
            if !start_line_set {
                start_line = i + 1
            }
        } else if !(c == ' ' as char) && !start_line_set {
            start_line_set = true;
        }
    }

    let total_lines = lines;
    lines = 0;

    for c in ASCII_ART.chars().skip(start_line) {
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
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("\n\nINIT phase start...\n\n");

    banner().await;

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
    let map = Arc::new(Mutex::new(HashMap::new()));
    let response_waiters: ResponseWaiters = Arc::new(Mutex::new(HashMap::new()));

    let webserver = TcpListener::bind(WEBSERVER_URL).await?;
    tokio::spawn(async move {
        let _ = axum::serve(webserver, r)
            .await
            .expect("Axum failed to start"); // start server w routers
    });
    let w_items: Vec<&str> = WEBSERVER_URL.split(":").collect();

    println!("Axum started on {} on port {}", w_items[0], w_items[1]);

    let t_items: Vec<&str> = TUNGSTENITE_URL.split(":").collect();

    println!(
        "Tokio Tungstenite started on {} on port {}",
        t_items[0], t_items[1]
    );

    let websocket = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("INIT phase successful");

    loop {
        let (stream, addr) = websocket.accept().await.unwrap();
        tokio::spawn(handle_connection(
            stream,
            addr,
            // switch these to a one shot or store them into a globle object?
            Arc::clone(&map),
            Arc::clone(&response_waiters),
            &arc_reg,
            Arc::clone(&kroushive_interface),
        ));
    }
}

// TODO optimise
async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: KuvasMap,
    response_waiters: ResponseWaiters,
    reg: &HiveHandlerRegistry,
    kroushive_interface: SharedHiveContext,
) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!(
            "Krousinator connected at {} on port {}",
            addr.ip(),
            addr.port()
        );

        let (mut write, mut read) = ws_stream.split();
        let id = Uuid::new_v4(); // auth before doing this please
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

                    let krous_env: KrousEnvelopeRecv =
                        match KrousEnvelopeRecv::deserialize(&raw_text, reg) {
                            Ok(val) => val,
                            Err(_) => {
                                println!("Found invalid JSON. Skipping.");
                                continue;
                            }
                        };

                    println!("{}", raw_text); // debugging

                    // let model_json = match json.get("model").and_then(|v| v.as_str()) {
                    //     Some(t) => t,
                    //     None => {
                    //         println!("No '_t' field found in model. Skipping.");
                    //         continue;
                    //     }
                    // };

                    // let message_type = match json.get("_t").and_then(|v| v.as_str()) {
                    //     Some(t) => t,
                    //     None => {
                    //         println!("No '_t' field found in model. Skipping.");
                    //         continue;
                    //     }
                    // };

                    // let model = match reg.get(message_type, &model_json) {
                    //     Some(Ok(model)) => model,
                    //     _ => continue,
                    // };

                    // let manual_request_id = json
                    //     .get("manual_request_id")
                    //     .and_then(|v| v.as_str())
                    //     .and_then(|s| Uuid::parse_str(s).ok());

                    if let Some(req_id) = krous_env.manual_request_id {
                        if let Some(waiter) = response_waiters.lock().await.remove(&req_id) {
                            let _ = waiter.send(krous_env.model); // context handles model
                        } else {
                            println!(
                                "manual request ID not found in hashmap which should be there."
                            )
                        }
                    } else {
                        krous_env
                            .model
                            .handle(Arc::clone(&kroushive_interface))
                            .await;
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
