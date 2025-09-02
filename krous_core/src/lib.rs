use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};

use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::{
    io,
    net::{TcpListener, TcpStream, unix},
    sync::{mpsc::UnboundedSender, Mutex},
};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::{
    context::shared::{KrousEnvelopeRecv, KrousHiveMeta},
    registry::registry::HiveHandlerRegistry,
    types::{KuvasMap, ResponseWaiters},
};

pub mod api;
pub mod context;
pub mod registry;
pub mod types;

static CLIENT_MAP: Lazy<Arc<Mutex<HashMap<Uuid, UnboundedSender<Message>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[rustfmt::skip]
static RESPONSE_WAITERS: Lazy<Arc<Mutex<HashMap<Uuid, tokio::sync::oneshot::Sender<KrousEnvelopeRecv>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub struct KrousHiveCore {
    pub router: axum::Router,
    pub registry: registry::registry::HiveHandlerRegistry,
    krous_websocket: Option<TcpListener>,
}

impl KrousHiveCore {
    pub fn new() -> Self {
        let router = api::axum::register_axum_handlers();
        let registry = registry::init_kroushive_registry();

        return Self {
            router,
            registry,
            krous_websocket: None,
        };
    }

    pub async fn start_axum(self, url: &str) -> Result<(), std::io::Error> {
        let websocket = TcpListener::bind(url).await?;
        tokio::spawn(async move {
            let _ = axum::serve(websocket, self.router)
                .await
                .expect("Axum failed to start"); // start server w routers
        });
        Ok(())
    }

    pub async fn start_krous(mut self, url: &str) -> Result<(), std::io::Error> {
        self.krous_websocket = Some(TcpListener::bind(url).await?);
        Ok(())
    }

    pub async fn check_new_connection(self: Arc<Self>) -> Result<(), std::io::Error> {
        let me = Arc::clone(&self);

        if let Some(listener) = &self.krous_websocket {
            let (stream, addr) = listener.accept().await?;

            tokio::spawn(KrousHiveCore::handle_connection(Arc::clone(&self), stream, addr));
        } else {
            println!("WebSocket listener not initialized yet. Did you forget to call KrousHiveCore.start_krous()?");
        }

        Ok(())
    }

    fn new_context(meta: KrousHiveMeta) -> context::hive_context::HiveContext {
        context::hive_context::HiveContext::new(meta)
    }

    async fn handle_connection(
        self: Arc<Self>,
        stream: TcpStream,
        addr: SocketAddr,
    ) {
        if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
            println!(
                "Krousinator connected at {} on port {}",
                addr.ip(),
                addr.port()
            );

            let (mut write, mut read) = ws_stream.split();
            let id = Uuid::new_v4(); // auth before doing this please
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

            (*CLIENT_MAP).lock().await.insert(id, tx);

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
                            Ok(text) => text.to_string(),
                            Err(e) => {
                                eprintln!("Failed to decode message text: {}", e);
                                continue;
                            }
                        };

                        let krous_env: KrousEnvelopeRecv =
                            match KrousEnvelopeRecv::deserialize(&raw_text, &self.registry) {
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
                            if let Some(waiter) = RESPONSE_WAITERS.lock().await.remove(&req_id) {
                                let _ = waiter.send(krous_env); // context handles model
                            } else {
                                println!(
                                    "manual request ID not found in hashmap which should be there."
                                )
                            }
                        } else {
                            let (model, meta) = krous_env.split();

                            

                            model
                                .handle(KrousHiveCore::new_context(meta))
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
            CLIENT_MAP.lock().await.remove(&id);
            write_task.abort();
        }
    }
}
