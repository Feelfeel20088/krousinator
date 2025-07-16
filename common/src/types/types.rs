use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::registry::HiveHandleable;

pub type KuvasMap = Arc<Mutex<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Message>>>>;
pub type ResponseWaiters = Arc<
    Mutex<
        HashMap<
            Uuid,
            tokio::sync::oneshot::Sender<Box<dyn HiveHandleable + 'static + Send + Sync>>,
        >,
    >,
>;
