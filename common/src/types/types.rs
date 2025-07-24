use std::{collections::HashMap, sync::Arc};

use crate::registry::HiveContext;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

pub type SharedHiveContext = Arc<Mutex<HiveContext>>;
pub type KuvasMap = Arc<Mutex<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Message>>>>;
pub type ResponseWaiters = Arc<Mutex<HashMap<Uuid, tokio::sync::oneshot::Sender<Value>>>>;
