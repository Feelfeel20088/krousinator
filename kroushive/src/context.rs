// add more things as needed it should be meta data that needs to be included in
// every req to both krousinator and kroushive
#[derive(Serialize, Deserialize)]
pub struct KrousEnvelopeSend<T> {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,
    #[serde(skip_deserializing)]
    pub model: T,
}
#[derive(serde::Deserialize)]
struct KrousEnvelopeHelper {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,
    pub model: String,
}

pub struct KrousEnvelopeRecv {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,

    pub model: Box<dyn HiveHandleable + Send + Sync>,
}

impl KrousEnvelopeRecv {
    fn deserialize(deserializer: String, reg: &HiveHandlerRegistry) -> Result<Self, Error> {
        let helper = match serde_json::from_str::<KrousEnvelopeHelper>(&deserializer) {
            Ok(helper) => helper,
            Err(e) => return Err(e),
        };

        // You can switch based on `_t` or just use a default model
        let model: Box<dyn HiveHandleable + Send + Sync> = match reg.get(&helper._t, &helper.model)
        {
            "noop" => Box::new(NoopHandler),
            _ => Box::new(NoopHandler), // fallback
        };

        Ok(KrousEnvelopeRecv {
            manual_request_id: helper.manual_request_id,
            id: helper.id,
            _t: helper._t,
            model,
        })
    }
}

impl<T> KrousEnvelopeSend<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn new(manual_request_id: Option<Uuid>, id: Uuid, _t: String, model: T) -> Self {
        Self {
            manual_request_id,
            id,
            _t,
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
}

pub struct HiveContext {}

impl HiveContext {
    pub async fn send_request_to_krousinator<T>(
        krousinator_id: Uuid,
        client_map: KuvasMap,
        response_waiters: ResponseWaiters,
        payload: T,
        type_name: String,
    ) -> Result<Box<dyn HiveHandleable + Send + Sync>, (StatusCode, String)>
    where
        T: Serialize + Send + Sync + 'static,
    {
        let request_id = Uuid::new_v4();
        let (tx, rx) = tokio::sync::oneshot::channel::<Box<dyn HiveHandleable + Send + Sync>>();

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
