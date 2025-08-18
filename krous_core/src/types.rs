use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::{
    api::model::traits::handlers::HiveHandleable,
    context::{hive_context::HiveContext, shared::KrousEnvelopeRecv},
};

pub type SharedHiveContext = Arc<Mutex<HiveContext>>;

pub type KuvasMap = Arc<Mutex<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Message>>>>;

pub type ResponseWaiters =
    Arc<Mutex<HashMap<Uuid, tokio::sync::oneshot::Sender<KrousEnvelopeRecv>>>>;
