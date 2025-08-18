use std::time::Duration;

use axum::http::StatusCode;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    api::model::{meta, traits::handlers::HiveHandleable},
    context::shared::{KrousEnvelopeRecv, KrousEnvelopeSend},
    types::{KuvasMap, ResponseWaiters},
};

pub struct HiveContext {
    meta: KrousEnvelopeRecv,
}

impl HiveContext {
    fn get_krousid(&self) -> Uuid {
        return self.meta.id;
    }

    pub async fn send_request_to_krousinator<T>(
        krousinator_id: Uuid,
        client_map: KuvasMap,
        response_waiters: ResponseWaiters,
        payload: T,
        type_name: String,
    ) -> Result<KrousEnvelopeRecv, (StatusCode, String)>
    where
        T: Serialize + Send + Sync + 'static,
    {
        let request_id = Uuid::new_v4();
        let (tx, rx) = tokio::sync::oneshot::channel::<KrousEnvelopeRecv>();

        // Register yourself as a waiter for this request ID
        response_waiters.lock().await.insert(request_id, tx);

        // Wrap payload into envelope
        let envelope = KrousEnvelopeSend::new(Some(request_id), krousinator_id, type_name, payload);

        // Serialize the envelope
        let s = match envelope.serd() {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

        let msg = tokio_tungstenite::tungstenite::Message::Text(s.into());

        // Send the model to the correct krousinator
        if let Some(krousinator_tx) = client_map.lock().await.get(&krousinator_id) {
            if let Err(e) = krousinator_tx.send(msg) {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to send message: {}", e),
                ));
            }
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Krousinator with id {} not found", krousinator_id),
            ));
        }

        // Wait for response
        let response = match tokio::time::timeout(Duration::from_secs(60), rx).await {
            Ok(Ok(response_value)) => response_value,
            Ok(Err(recv_err)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to receive response: {}", recv_err),
                ));
            }
            Err(elapsed) => {
                return Err((
                    StatusCode::REQUEST_TIMEOUT,
                    format!("Request timed out after {}", elapsed),
                ));
            }
        };

        Ok(response)
    }
}
