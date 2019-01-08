use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Error};
use std::string::FromUtf8Error;

pub struct PacketReader {
    data: Cursor<Vec<u8>>,
}

impl PacketReader {
    pub fn new(bytes_array: Vec<u8>) -> PacketReader {
        PacketReader {
            data: Cursor::new(bytes_array),
        }
    }

    pub fn get_packet_id(&mut self) -> u16 {

        self.data.read_u16::<LittleEndian>().unwrap()
    }

    // Read unsigned integer int
    pub fn read_u8(&mut self) -> u8 {
        self.data.read_u8().unwrap()
    }

    pub fn read_u16(&mut self) -> u16 {
        self.data.read_u16::<LittleEndian>().unwrap()
    }

    pub fn read_u24(&mut self) -> u32 {
        self.data.read_u24::<LittleEndian>().unwrap()
    }

    pub fn read_u32(&mut self) -> u32 {
        self.data.read_u32::<LittleEndian>().unwrap()
    }

    pub fn read_u48(&mut self) -> u64 {
        self.data.read_u48::<LittleEndian>().unwrap()
    }

    pub fn read_u64(&mut self) -> u64 {
        self.data.read_u64::<LittleEndian>().unwrap()
    }

    // Read signed integer int
    pub fn read_i8(&mut self) -> i8 {
        self.data.read_i8().unwrap()
    }

    pub fn read_i16(&mut self) -> i16 {
        self.data.read_i16::<LittleEndian>().unwrap()
    }

    pub fn read_i24(&mut self) -> i32 {
        self.data.read_i24::<LittleEndian>().unwrap()
    }

    pub fn read_i32(&mut self) -> i32 {
        self.data.read_i32::<LittleEndian>().unwrap()
    }

    pub fn read_i48(&mut self) -> i64 {
        self.data.read_i48::<LittleEndian>().unwrap()
    }

    pub fn read_i64(&mut self) -> i64 {
        self.data.read_i64::<LittleEndian>().unwrap()
    }

    // Read float
    pub fn read_f32(&mut self) -> f32 {
        self.data.read_f32::<LittleEndian>().unwrap()
    }

    pub fn read_f64(&mut self) -> f64 {
        self.data.read_f64::<LittleEndian>().unwrap()
    }

    // Read string
    pub fn read_string(&mut self) -> String {
        let mut chars = Vec::new();
        loop {
            let char_code = self.data.read_u8().unwrap();
            let is_end_of_string = char_code == 0;

            if is_end_of_string {
                break;
            }

            chars.push(char_code);
        }

        String::from_utf8(chars).unwrap()
    }

    pub fn read_bool(&mut self) -> bool {
        self.data.read_u8().unwrap() == 1
    }
}
