use byteorder::{BigEndian, ReadBytesExt};
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

    pub fn get_packet_id(&mut self) -> Result<u16, Error> {
        self.data.read_u16::<BigEndian>()
    }

    // Read unsigned integer int
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        self.data.read_u8()
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        self.data.read_u16::<BigEndian>()
    }

    pub fn read_u24(&mut self) -> Result<u32, Error> {
        self.data.read_u24::<BigEndian>()
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        self.data.read_u32::<BigEndian>()
    }

    pub fn read_u48(&mut self) -> Result<u64, Error> {
        self.data.read_u48::<BigEndian>()
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        self.data.read_u64::<BigEndian>()
    }

    // Read signed integer int
    pub fn read_i8(&mut self) -> Result<i8, Error> {
        self.data.read_i8()
    }

    pub fn read_i16(&mut self) -> Result<i16, Error> {
        self.data.read_i16::<BigEndian>()
    }

    pub fn read_i24(&mut self) -> Result<i32, Error> {
        self.data.read_i24::<BigEndian>()
    }

    pub fn read_i32(&mut self) -> Result<i32, Error> {
        self.data.read_i32::<BigEndian>()
    }

    pub fn read_i48(&mut self) -> Result<i64, Error> {
        self.data.read_i48::<BigEndian>()
    }

    pub fn read_i64(&mut self) -> Result<i64, Error> {
        self.data.read_i64::<BigEndian>()
    }

    // Read float
    pub fn read_f32(&mut self) -> Result<f32, Error> {
        self.data.read_f32::<BigEndian>()
    }

    pub fn read_f64(&mut self) -> Result<f64, Error> {
        self.data.read_f64::<BigEndian>()
    }

    // Read string
    pub fn read_string(&mut self) -> Result<String, FromUtf8Error> {
        let mut chars = Vec::new();
        loop {
            let char_code = self.data.read_u8().unwrap();
            let is_end_of_string = char_code == 0;

            if is_end_of_string {
                break;
            }

            chars.push(char_code);
        }

        String::from_utf8(chars)
    }

    pub fn read_bool(&mut self) -> bool {
        self.data.read_u8().unwrap() == 1
    }
}
