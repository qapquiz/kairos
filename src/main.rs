mod packet_writer;
mod packet;
mod packet_reader;
mod server;
mod remote;

use self::remote::Remote;
use self::server::Server;
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};
use ws::listen;

fn main() {
    let ip_address = "127.0.0.1:8000";
    let clients = Arc::new(RwLock::new(HashMap::new()));
    let mut id = Arc::new(Mutex::new(0));

    let server_thread = thread::spawn(move || {
        let clients = clients.clone();
        let mut id_clone = id.lock().unwrap();

        listen(ip_address, |ws| {
            let mut write_clients = clients.write().unwrap();
            *id_clone += 1;
            write_clients.insert(id_clone.clone(), Remote { ws: ws.clone() });
            drop(write_clients);

            Server {
                remote: Remote {
                    ws
                },
                remotes: Arc::clone(&clients)
            }
        }).unwrap()
    });

    sleep(Duration::from_millis(10));

    let _ = server_thread.join();
}

//fn main() {
//    let mut remotes: Arc<HashMap<u32, &Socket>> = Arc::new(HashMap::new());
//
//    start_server_with_thread(String::from("127.0.0.1:3012"), &mut remotes);
//}
//
//fn start_server() {
////    start_server_with_thread(String::from("127.0.0.1:3012"));
//    start_server_without_thread(String::from("127.0.0.1:3012"));
//}
//
//fn start_server_with_thread(url: String, remotes: &'static mut Arc<HashMap<u32, &Socket>>) {
//    let server =
//        thread::spawn(move || listen(url, |socket| {
//            let socket = Socket::new(socket);
//            remotes.insert(1, &socket);
//            socket
//        }).unwrap());
//
//    sleep(Duration::from_millis(10));
//
//    let _ = server.join();
//}
//
//fn start_server_without_thread(url: String) {
//    if let Err(error) = listen(url, |socket| {
////        let socket = Socket::new(socket);
////        unsafe {
////            remotes.insert(id, &socket);
////        }
////        socket.value.broadcast()
//        Socket::new(socket)
//    }) {
//        println!("Failed to create WebSocket due to {:?}", error);
//    }
//}
