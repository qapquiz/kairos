use crate::packet;
use crate::remote::Remote;
use std::fmt::Display;
use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
use ws::{CloseCode, Handler, Message, Result, Sender, Handshake};

#[derive(Debug)]
pub struct Server {
    pub remote: Remote,
    pub remotes: Arc<RwLock<HashMap<i32, Remote>>>
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Client connected");
        Ok(())
    }


    fn on_message(&mut self, msg: Message) -> Result<()> {
        let data = match msg {
            Message::Text(_) => panic!("Client must send message as Vec<u8> only "),
            Message::Binary(binary) => binary,
        };

        packet::receive_packet(&self.remote, &self.remotes, data);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Client disconnect with CloseCode: {:?}", code);
    }
}
