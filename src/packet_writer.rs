use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Result;

pub struct PacketWriter {
    pub buffer: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> PacketWriter {
        PacketWriter { buffer: Vec::new() }
    }

    pub fn write_packet_id(&mut self, packet_id: u16) {
        self.buffer.write_u16::<LittleEndian>(packet_id).unwrap();
    }

    // Write unsigned integer int
    pub fn write_u8(&mut self, number: u8) -> Result<()> {
        self.buffer.write_u8(number)
    }

    pub fn write_u16(&mut self, number: u16) -> Result<()> {
        self.buffer.write_u16::<LittleEndian>(number)
    }

    pub fn write_u24(&mut self, number: u32) -> Result<()> {
        self.buffer.write_u24::<LittleEndian>(number)
    }

    pub fn write_u32(&mut self, number: u32) -> Result<()> {
        self.buffer.write_u32::<LittleEndian>(number)
    }

    pub fn write_u48(&mut self, number: u64) -> Result<()> {
        self.buffer.write_u48::<LittleEndian>(number)
    }

    pub fn write_u64(&mut self, number: u64) -> Result<()> {
        self.buffer.write_u64::<LittleEndian>(number)
    }

    // Write signed integer int
    pub fn write_i8(&mut self, number: i8) -> Result<()> {
        self.buffer.write_i8(number)
    }

    pub fn write_i16(&mut self, number: i16) -> Result<()> {
        self.buffer.write_i16::<LittleEndian>(number)
    }

    pub fn write_i24(&mut self, number: i32) -> Result<()> {
        self.buffer.write_i24::<LittleEndian>(number)
    }

    pub fn write_i32(&mut self, number: i32) -> Result<()> {
        self.buffer.write_i32::<LittleEndian>(number)
    }

    pub fn write_i48(&mut self, number: i64) -> Result<()> {
        self.buffer.write_i48::<LittleEndian>(number)
    }

    pub fn write_i64(&mut self, number: i64) -> Result<()> {
        self.buffer.write_i64::<LittleEndian>(number)
    }

    // Write float
    pub fn write_f32(&mut self, number: f32) -> Result<()> {
        self.buffer.write_f32::<LittleEndian>(number)
    }

    pub fn write_f64(&mut self, number: f64) -> Result<()> {
        self.buffer.write_f64::<LittleEndian>(number)
    }

    pub fn write_string(&mut self, message: String) -> Result<()> {
        let bytes = message.as_bytes();

        for (index, &char_code) in bytes.iter().enumerate() {
            self.buffer.write_u8(char_code).unwrap();
        }

        self.buffer.write_u8(0)
    }

    pub fn write_bool(&mut self, boolean: bool) -> Result<()> {
        if boolean {
            self.buffer.write_u8(1)
        } else {
            self.buffer.write_u8(0)
        }
    }
}
