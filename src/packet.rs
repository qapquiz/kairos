use crate::server::Server;
use crate::packet_reader::PacketReader;
use crate::packet_writer::PacketWriter;
use crate::remote::Remote;

use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
use std::any::Any;
use num;
use num_derive::FromPrimitive;

macro_rules! write_packet {
    ( $( $item:expr ), * ) => {
        {
            let mut packet_writer = PacketWriter::new();
            $(
                if let Some(number) = (&$item as &Any).downcast_ref::<u8>() {
                    packet_writer.write_u8(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<u16>() {
                    packet_writer.write_u16(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<u32>() {
                    packet_writer.write_u24(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<u32>() {
                    packet_writer.write_u32(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<u64>() {
                    packet_writer.write_u64(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<i8>() {
                    packet_writer.write_i8(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<i16>() {
                    packet_writer.write_i16(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<i32>() {
                    packet_writer.write_i24(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<i32>() {
                    packet_writer.write_i32(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<i64>() {
                    packet_writer.write_i64(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<f32>() {
                    packet_writer.write_f32(number.to_owned()).unwrap();
                } else if let Some(number) = (&$item as &Any).downcast_ref::<f64>() {
                    packet_writer.write_f64(number.to_owned()).unwrap();
                } else if let Some(message) = (&$item as &Any).downcast_ref::<String>() {
                    packet_writer.write_string(message.to_owned()).unwrap();
                } else if let Some(true_or_false) = (&$item as &Any).downcast_ref::<bool>() {
                    packet_writer.write_bool(true_or_false.to_owned()).unwrap();
                }
            )*
            packet_writer
        }
    };
}

#[derive(FromPrimitive)]
pub enum Packet {
    CSLogIn = 10001,

    SCLoggedIn = 20001,
}

//pub fn receive_packet(client_sender: &websocket::sender::Writer<std::net::TcpStream>, data: Vec<u8>) {
//    let mut reader = PacketReader::new(data);
//    let packet_id = reader.get_packet_id();
//
//    println!("packet_id: {}", packet_id);
//}

pub fn receive_packet(remote: &Remote, remotes: &Arc<RwLock<HashMap<i32, Remote>>>, data: Vec<u8>) {
    let mut reader =  PacketReader::new(data);
    let packet_id = reader.get_packet_id();

    let packet = num::FromPrimitive::from_u16(packet_id);

    match packet {
        Some(Packet::CSLogIn) => {
            let name = reader.read_string();
            let age = reader.read_i8();

            println!("name: {}", name);
            println!("age: {}", age);

            let read_remote = remotes.read().unwrap();

            if let Some(client) = read_remote.get(&1) {
                let mut writer = PacketWriter::new();

                writer.write_packet_id(Packet::SCLoggedIn as u16);
                writer.write_string(String::from("SERVER"));
                writer.write_i8(20);
                client.send(writer.buffer);
            }
//
//            let mut writer = PacketWriter::new();
//            writer.write_packet_id(Packet::SCLoggedIn as u16);
//            writer.write_string(String::from("SERVER"));
//            writer.write_i8(20);
//
//            remote.send(writer.buffer);
//            let test = 10 as u32;
//            let packet_writer = write!(Packet::SCLoggedIn as u16, 2, 30, 40, "stirwofwp", false);
//            socket.send(packet_writer.buffer);
//            println!("Hello this is Packet::CSLogIn");
//            let name = reader.read_string();
//            println!("read string: {}", name);
//
//            let mut writer = PacketWriter::new();
//            writer.write_packet_id(20001);
//            writer.write_string(name);
//            println!("{:?}", writer.buffer);
//            socket.value.send(writer.buffer.clone());
//            socket.value.send(writer.buffer.clone());
        },

        None => panic!("Unknown packet id: {}", packet_id),
        _ => panic!("This is not receive packet maybe you use wrong packet_id (packet_id: {})", packet_id),
    }
}

//pub fn send_packet(socket: &mut Socket, packet: Packet) {
//    match packet {
//        Packet::SCLoggedIn => {
//            let writer = PacketWriter::new();
//
//        },
//
//        _ => panic!("This is not send packet maybe you want to use receive_packet() function instead"),
//    }
//}
