use futures_util::SinkExt;
use tokio::sync::mpsc::{channel, Sender};
use uuid::Uuid;

type WebsocketWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;

pub struct Context {
    sender: Sender<String>,
    uuid: Uuid,
}
//
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

    pub async fn send<T>(&self, send_object: T)
    where
        T: serde::Serialize + std::marker::Send + 'static,
    {
        let sender_clone = self.sender.clone();
        let payload = serde_json::to_string(&send_object).unwrap_or_default();
        let _ = sender_clone.send(payload).await;
    }

    pub fn set_uuid(&mut self, id: Uuid) {
        self.uuid = id;
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}
