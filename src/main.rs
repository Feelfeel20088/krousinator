mod registry;
mod models;
use crate::registry::{krousinator_interface::KrousinatorInterface, handler_registry::HandlerRegistry};
use crate::registry::entry::HandlerMeta;

use serde_json::Value;


#[tokio::main]
async fn main() {


    let mut krous = KrousinatorInterface::new();
    let mut reg: HandlerRegistry = HandlerRegistry::new();


    for handler in inventory::iter::<HandlerMeta> {
        reg.register(handler.name, handler.constructor);
    }



    let incoming = vec![
        (r#"{"t": "HandleTest", "msg": "Hello from ping"}"#),
    ];
    let value: Value = serde_json::from_str(incoming[0]).unwrap();
    if let Some(t_str) = value.get("t").and_then(|v| v.as_str()) {
        println!("Type field: {}", t_str);
        let s = reg.get(t_str, incoming[0]).unwrap_or_else(|| panic!("uh oh krousy"));
        s.handle(&mut krous);
    }

    // use crate::models::recv::handle::Handleable;
    // use serde::de::DeserializeOwned;
    
    // pub type DynHandlerConstructor = fn(&str) -> Box<dyn Handleable>;
    
    // pub struct HandlerMeta {
    //     pub name: &'static str,
    //     pub constructor: DynHandlerConstructor,
    // }
    
    // inventory::collect!(HandlerMeta);


    // let (tx, mut rx) = mpsc::channel(32);
    // // Establish connection
    // let tx_clone = tx.clone();
    // tokio::spawn(async move {
    //     for i in 0..5 {
    //         tx_clone.send(()).await.unwrap();
            
    //     }
    // });

    // let (mut ws_stream, _) = connect_async("wss://ws.felixhub.dev").await.expect("Failed to connect");

    // println!("✅ Connected!");
    // let (mut write, mut read) = ws_stream.split();
    // let write_object = Arc::new(write);
    

    // write_object.clone();

    // while let Some(msg) = rx.recv().await {
    //     // println!("📨 Received: {}", msg);
    // }

    // // 📨 Send a message to the server
    // let msg = Message::Text("Hello from Rust client!".into());
    // write_object.send(msg).await.unwrap();
    // println!("📤 Sent message");

    // // 📥 Read message from the server
    // if let Some(Ok(msg)) = read.next().await {
    //     println!("📬 Received: {}", msg);
    // }

    // println!("👋 Done!");
}
