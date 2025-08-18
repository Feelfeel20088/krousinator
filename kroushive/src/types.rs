pub type SharedHiveContext = Arc<Mutex<HiveContext>>;
pub type KuvasMap = Arc<Mutex<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Message>>>>;
pub type ResponseWaiters =
    Arc<Mutex<HashMap<Uuid, tokio::sync::oneshot::Sender<Box<dyn HiveHandleable + Send + Sync>>>>>;
