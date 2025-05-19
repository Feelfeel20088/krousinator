use crate::models::send::send_model_traits::Producer;
use futures_util::SinkExt;
use serde_json;
use tokio::sync::mpsc::{Sender, channel};

type WebsocketWriter = futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>;


pub struct KrousinatorInterface {
    sender: Sender<String>,
}

impl Clone for KrousinatorInterface {
    fn clone(&self) -> Self {
        KrousinatorInterface {
            sender: self.sender.clone(),
        }
    }
}


impl KrousinatorInterface {
    pub fn new(mut write: WebsocketWriter) -> Self {

        let (tx, mut rx) = channel::<String>(100);

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = write.send(msg.into()).await;
            }
        });

        KrousinatorInterface {sender: tx}
    }

    pub fn send<T>(&self, send_object: T) 
        where
            T: Producer + serde::Serialize + std::marker::Send + 'static,
    {   
        let sender_clone = self.sender.clone();
        tokio::spawn(async move {
            let payload = serde_json::to_string(&send_object).unwrap_or_default();
            let _ = sender_clone.send(payload).await;
        });

    }

}