use crate::registry::{HiveHandleable, HiveProducer};

use crate::types::ResponseWaiters;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::SinkExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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

// add more things as needed it should be meta data that needs to be included in
// every req to both krousinator and kroushive
#[derive(Serialize, Deserialize)]
pub struct KrousEnvelopeSend<T> {
    pub manual_request_id: Option<Uuid>,
    pub model: T,
}

impl<T> KrousEnvelopeSend<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn new(manual_request_id: Option<Uuid>, model: T) -> Self {
        Self {
            manual_request_id,
            model,
        }
    }

    fn serd(self) -> Result<String, (StatusCode, std::string::String)> {
        match serde_json::to_string(&self) {
            Ok(inner) => Ok(inner),
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Model sent is not valid json".to_string(),
                ))
            }
        }
    }

    fn dserd(v: Value) -> Result<KrousEnvelopeSend<T>, ()> {
        match serde_json::from_value::<Self>(v) {
            Ok(inner) => Ok(inner),
            Err(_) => return Err(()),
        }
    }
}

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
    pub async fn send_request_to_krousinator<T, T2>(
        krousinator_id: Uuid,
        client_map: KuvasMap,
        response_waiters: ResponseWaiters,
        payload: T,
    ) -> Result<T2, impl IntoResponse>
    where
        T: HiveProducer + Serialize + DeserializeOwned + Send + Sync + 'static,
        T2: HiveHandleable + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let request_id = Uuid::new_v4();
        let (tx, rx) = tokio::sync::oneshot::channel::<Value>();
        // Register yourself as a waiter for this request ID
        response_waiters.lock().await.insert(request_id, tx);
        let envelope = KrousEnvelopeSend::new(Some(request_id), payload);

        let s = match envelope.serd() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let msg = Message::Text(s.into());

        // send the model to the correct krousinator
        if let Some(krousinator_tx) = client_map.lock().await.get(&krousinator_id) {
            krousinator_tx.send(msg).unwrap();
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Krousinator with id {} is not found", &krousinator_id),
            ));
        }

        // Wait for response
        let response: KrousEnvelopeSend<T2> =
            match tokio::time::timeout(Duration::from_secs(60), rx).await {
                Ok(Ok(response)) => match KrousEnvelopeSend::dserd(response) {
                    Ok(model) => model,
                    Err(_) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to serulize response from krousinator"),
                        ))
                    }
                },

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

        return Ok(response.model);
    }
}
