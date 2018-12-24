mod packet_writer;
mod packet;
mod packet_reader;
mod socket;
mod socket_server;

use self::socket::Socket;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use ws::listen;

fn main() {
    let server =
        thread::spawn(move || listen("127.0.0.1:3012", |socket| Socket::new(socket)).unwrap());

    sleep(Duration::from_millis(10));

    let _ = server.join();
}
