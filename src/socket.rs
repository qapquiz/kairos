use crate::packet;
use ws::{CloseCode, Handler, Message, Result, Sender};

pub struct Socket {
    pub value: Sender,
}

impl Handler for Socket {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let data = match msg {
            Message::Text(_) => panic!("Client must send message as Vec<u8> only "),
            Message::Binary(binary) => binary,
        };

        packet::receive_packet(self, data);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Client disconnect with CloseCode: {:?}", code);
    }
}

impl Socket {
    pub fn new(sender: Sender) -> Socket {
        Socket { value: sender }
    }
}
