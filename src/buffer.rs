use core::str;
use std::fmt::{self, Display};
use serde::{de, ser};
use thiserror::Error;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

static SEGMENT_BITS:u8 = 0x7F;
static CONTINUE_BIT:u8 = 0x80;

#[derive(Error,Debug,Clone)]
pub enum BufferError{
    #[error("Serialization Error: {0}")]
    SerializerMessage(String),
    #[error("Serialization Error: {0}")]
    DeserializerMessage(String)
}

impl de::Error for BufferError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::DeserializerMessage(msg.to_string())
    }
}

impl ser::Error for BufferError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::SerializerMessage(msg.to_string())
    }
}


#[derive(Debug,Clone)]
 pub struct ProtocolBuffer {
    pub buffer: BytesMut
 }

 impl ProtocolBuffer {
    pub fn new(buffer:BytesMut) -> Self {
        Self{
            buffer
        }
    }

    pub fn read_i8(&mut self) -> Result<i8,BufferError> {
        if self.buffer.has_remaining() {
            Ok(self.buffer.get_i8())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read i8!".to_owned()))
        }
    }

    pub fn read_u8(&mut self) -> Result<u8,BufferError> {
        if self.buffer.has_remaining() {
            Ok(self.buffer.get_u8())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }   
    

    pub fn read_var_int(&mut self) -> Result<i32,BufferError> {
        let mut value:i32 = 0;
        let mut position:i32 = 0;
        let mut current_byte:u8 = 0;

        loop {
            current_byte = self.read_u8()?;

            value |= ((current_byte & SEGMENT_BITS as u8) as i32) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            
            position += 7;

            if position >= 32 {
                return Err(BufferError::DeserializerMessage("VarInt is too big".to_owned()));
            }
        }

        return Ok(value)
    }

    pub fn read_var_long(&mut self) -> Result<i64,BufferError> {
        let mut value:i64 = 0;
        let mut position:i32 = 0;
        let mut current_byte:u8 = 0;

        loop {
            current_byte = self.read_u8()?;

            value |= ((current_byte & SEGMENT_BITS as u8) as i64) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            
            position += 7;

            if position >= 64 {
                return Err(BufferError::DeserializerMessage("VarLong is too big".to_owned()));
            }
        }

        return Ok(value);
    }
    
    pub fn read_full_string(&mut self) -> Result<String,BufferError> {
        self.read_string(i16::MAX.into())
    }

    pub fn read_string(&mut self,size:i32) -> Result<String,BufferError> {
        let buf = self.read_var_int()?;

        if buf > size {
            return Err(BufferError::DeserializerMessage("String is bigger than the max size!".to_owned()));
        }

        let data = self.copy_to_bytes(size as usize)?;

        if data.len() as i32 > size {
            panic!("String is bigger than the max size!")
        }
        
        return Ok(str::from_utf8(&data).expect("Failed to convert byte data to utf8 string!").to_string());
    }

    pub fn read_option<T>(&mut self, function: impl Fn(&mut Self) -> T) -> Option<T> {
        Some(function(self))
    }


    pub fn read_uuid(&mut self) -> Result<Uuid,BufferError> {
        let high = self.read_u64()?;
        let low = self.read_u64()?;

        let mut bytes: [u8;16] = [0u8; 16];
        bytes[..8].copy_from_slice(&high.to_be_bytes());
        bytes[8..].copy_from_slice(&low.to_be_bytes());

        match Uuid::from_slice(&bytes) {
            Ok(value) => {return Ok(value)},
            Err(_) => {return Err(BufferError::DeserializerMessage("Failed to read UUID".to_owned()))}
        }
    }


    pub fn read_fixed_bitset(&mut self, bits:usize) -> Result<Bytes, BufferError> {
        return Ok(self.copy_to_bytes(bits.div_ceil(8))?)
    }

    
    pub fn read_bool(&mut self) -> Result<bool, BufferError> {
        return Ok(self.read_u8()? != 0);
    }

    pub fn read_i16(&mut self) -> Result<i16, BufferError> {
        if self.buffer.remaining() >= 2 {
            return Ok(self.buffer.get_i16());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read i16".to_owned()))
        }
    }
    
    pub fn read_u16(&mut self) -> Result<u16, BufferError> {
        if self.buffer.remaining() >= 2 {
            return Ok(self.buffer.get_u16());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u16".to_owned()))
        }
    }

    pub fn read_i32(&mut self) -> Result<i32, BufferError> {
        if self.buffer.remaining() >= 4 {
            return Ok(self.buffer.get_i32());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read i32".to_owned()))
        }
    }
    
    pub fn read_u32(&mut self) -> Result<u32, BufferError> {
        if self.buffer.remaining() >= 4 {
            return Ok(self.buffer.get_u32());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u32".to_owned()))
        }
    }

    pub fn read_i64(&mut self) -> Result<i64, BufferError> {
        if self.buffer.remaining() >= 8 {
            return Ok(self.buffer.get_i64());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read i64".to_owned()))
        }
    }

    pub fn read_u64(&mut self) -> Result<u64, BufferError> {
        if self.buffer.remaining() >= 8 {
            return Ok(self.buffer.get_u64());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u64".to_owned()))
        }
    }

    pub fn read_f32(&mut self) -> Result<f32, BufferError> {
        if self.buffer.remaining() >= 4 {
            return Ok(self.buffer.get_f32());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read f32".to_owned()))
        }
    }

    pub fn read_f64(&mut self) -> Result<f64, BufferError> {
        if self.buffer.remaining() >= 8 {
            return Ok(self.buffer.get_f64());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read f64".to_owned()))
        }
    }

    pub fn write_option<T>(&mut self, value:&Option<T>,function: impl Fn(&mut Self,&T)) {
        function(self,&value.as_ref().unwrap());
    }


    pub fn write_u8(&mut self,value:u8) {
        self.buffer.put_u8(value);
    }

    pub fn copy_to_bytes(&mut self,size:usize) -> Result<Bytes, BufferError> {
        if self.buffer.len() >= size {
            return Ok(self.buffer.copy_to_bytes(size))
        } else {
            Err(BufferError::DeserializerMessage("Failed to copy bytes!".to_owned()))
        }
    }

    pub fn copy_to_slice(&mut self,dst:&mut [u8]) -> Result<(),BufferError> {
        if self.buffer.remaining() >= dst.len() {
            self.buffer.copy_from_slice(dst);
            Ok(())
        } else {
            Err(BufferError::DeserializerMessage("Failed to copy from slice!".to_owned()))
        }
    }
 }

 
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    
    #[test]
    fn test_read_var_int() {
        let data:Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ProtocolBuffer::new(data_buf);

        assert_eq!(buffer.read_var_int().unwrap(),-2147483648)

    }

    #[test]
    fn test_read_var_long() {
        let data:Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ProtocolBuffer::new(data_buf);

        assert_eq!(buffer.read_var_long().unwrap(),-9223372036854775808);
    }

    #[test]
    fn test_read_string() {
        let data:Vec<u8> = vec![0x09, //String Size (9 bytes)
        0x6D, 0x69, 0x6E, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74]; //String Data

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ProtocolBuffer::new(data_buf);

        assert_eq!(buffer.read_string(9).unwrap(), "minecraft")

    }

    #[test]
    fn test_read_uuid() {
        let data: Vec<u8> = vec![
            0x55, 0x0e, 0x84, 0x00, // 4 bytes
            0xe2, 0x9b, // 2 bytes
            0x41, 0xd4, // 2 bytes
            0xa7, 0x16, // 2 bytes
            0x44, 0x66, 0x55, 0x44, 0x00, 0x00 // 6 bytes
        ];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ProtocolBuffer::new(data_buf);

        assert_eq!(buffer.read_uuid().unwrap(),Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }

    #[test]
    fn test_read_data_types() {
        let data: Vec<u8> = vec![
            // i8 and u8
            0x7F, // 127 (i8 and u8)
            0x80, // -128 (i8) and 128 (u8)

            // i16 and u16
            0x7F, 0xFF, // 32767 (i16, u16)
            0x80, 0x00, // -32768 (i16) and 32768 (u16)

            // i32 and u32
            0x7F, 0xFF, 0xFF, 0xFF, // 2147483647 (i32, u32)
            0x80, 0x00, 0x00, 0x00, // -2147483648 (i32) and 2147483648 (u32)

            // i64 and u64
            0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 9223372036854775807 (i64, u64)
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // -9223372036854775808 (i64) and 9223372036854775808 (u64)

            // f32
            0x3F, 0x80, 0x00, 0x00, // 1.0 (f32)

            // f64
            0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 1.0 (f64)

            // bool
            0x01, // true (bool)

            // bitset (example with 8 bits, which fits in one byte)
            0b10101010, // example bitset of 8 bits (170 in decimal)
        ];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ProtocolBuffer::new(data_buf);

        assert_eq!(buffer.read_i8().unwrap(),127);
        assert_eq!(buffer.read_u8().unwrap(),128);
        assert_eq!(buffer.read_i16().unwrap(),32767);
        assert_eq!(buffer.read_u16().unwrap(),32768);
        assert_eq!(buffer.read_i32().unwrap(),2147483647);
        assert_eq!(buffer.read_u32().unwrap(),2147483648);
        assert_eq!(buffer.read_i64().unwrap(),9223372036854775807);
        assert_eq!(buffer.read_u64().unwrap(),9223372036854775808);
        assert_eq!(buffer.read_f32().unwrap(),1.0);
        assert_eq!(buffer.read_f64().unwrap(),1.0);
        assert_eq!(buffer.read_bool().unwrap(),true);
    }
}


impl Default for ProtocolBuffer {
    fn default() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }
 }