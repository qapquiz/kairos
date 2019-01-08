use ws::{Sender};

#[derive(Debug)]
pub struct Remote {
    pub ws: Sender
}

impl Remote {
    pub fn send(&self, data: Vec<u8>) {
        self.ws.send(data);
    }
}