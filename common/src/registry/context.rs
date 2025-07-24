use crate::registry::HiveHandleable;
use crate::types::ResponseWaiters;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::SinkExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::Duration;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

type WebsocketWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;
use crate::types::KuvasMap;

pub struct Context {
    sender: Sender<String>,
    uuid: Uuid,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            sender: self.sender.clone(),
            uuid: self.uuid.clone(),
        }
    }
}

impl Context {
    pub fn new(mut write: WebsocketWriter) -> Self {
        let (tx, mut rx) = channel::<String>(100);

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = write.send(msg.into()).await;
            }
        });

        Context {
            sender: tx,
            uuid: Uuid::nil(),
        }
    }

    pub fn send<T>(&self, send_object: T)
    where
        T: serde::Serialize + std::marker::Send + 'static,
    {
        let sender_clone = self.sender.clone();
        tokio::spawn(async move {
            let payload = serde_json::to_string(&send_object).unwrap_or_default();
            let _ = sender_clone.send(payload).await;
        });
    }

    pub fn set_uuid(&mut self, id: Uuid) {
        self.uuid = id;
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}

pub struct HiveContext {}

impl HiveContext {
    pub async fn send_request_to_krousinator<T>(
        krousinator_id: Uuid,
        client_map: KuvasMap,
        response_waiters: ResponseWaiters,
        payload: String,
    ) -> Result<T, impl IntoResponse>
    where
        T: HiveHandleable + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        loop {
            let request_id = Uuid::new_v4();
            let (tx, rx) = tokio::sync::oneshot::channel::<Value>();

            // Register yourself as a waiter for this request ID
            response_waiters.lock().await.insert(request_id, tx);

            // payload is a allredy serded string produced in the build_handler function
            let msg = Message::Text(payload.into());

            // send the model to the correct krousinator
            if let Some(krousinator_tx) = client_map.lock().await.get(&krousinator_id) {
                krousinator_tx.send(msg).unwrap();
            } else {
                return Err((
                    StatusCode::NOT_FOUND,
                    format!("Krousinator with id {} is not found", &krousinator_id),
                ));
            }

            // Wait for response (timeout optional)
            let response = match tokio::time::timeout(Duration::from_secs(60), rx).await {
                Ok(Ok(response)) => {
                    let model: T = match serde_json::from_value::<T>(response) {
                        Ok(model) => model,
                        Err(e) => {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to serulize response from krousinator: {}", e),
                            ))
                        }
                    };
                    model
                }

                Ok(Err(recv_err)) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to receive response: {}", recv_err),
                    ))
                }

                Err(elapsed) => {
                    return Err((
                        StatusCode::REQUEST_TIMEOUT,
                        format!(
                            "Failed to receive response, request timed out in {}",
                            elapsed.to_string()
                        ),
                    ))
                }
            };

            return Ok(response);
        }
    }
}
