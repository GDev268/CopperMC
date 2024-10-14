use core::str;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

use crate::deserializer::SerializerError;

static SEGMENT_BITS:u8 = 0x7F;
static CONTINUE_BIT:u8 = 0x80;

#[derive(Debug,Clone)]
 pub struct ByteBuffer {
    pub buffer: BytesMut
 }

 impl ByteBuffer {
    pub fn new(buffer:BytesMut) -> Self {
        Self{
            buffer
        }
    }

    pub fn read_i8(&mut self) -> Result<i8,SerializerError> {
        if self.buffer.has_remaining() {
            Ok(self.buffer.get_i8())
        } else {
            Err(SerializerError::SerializerMessage("Failed to read i8!".to_owned()))
        }
    }

    pub fn read_u8(&mut self) -> Result<u8,SerializerError> {
        if self.buffer.has_remaining() {
            Ok(self.buffer.get_u8())
        } else {
            Err(SerializerError::SerializerMessage("Failed to read u8!".to_owned()))
        }
    }


    pub fn read_var_int(&mut self) -> Result<i32,SerializerError> {
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
                return Err(SerializerError::SerializerMessage("VarInt is too big".to_owned()));
            }
        }

        return Ok(value)
    }

    pub fn read_var_long(&mut self) -> Result<i64,SerializerError> {
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
                return Err(SerializerError::SerializerMessage("VarLong is too big".to_owned()));
            }
        }

        return Ok(value);
    }
    
    pub fn read_string(&mut self,size:i32) -> Result<String,SerializerError> {
        let buf = self.read_var_int()?;

        if buf > size {
            return Err(SerializerError::SerializerMessage("String is bigger than the max size!".to_owned()));
        }

        let data = self.copy_to_bytes(size as usize)?;

        if data.len() as i32 > size {
            panic!("String is bigger than the max size!")
        }
        
        return Ok(str::from_utf8(&data).expect("Failed to convert byte data to utf8 string!").to_string());
    }

    pub fn read_uuid(&mut self) -> Result<Uuid,SerializerError> {
        let high = self.read_u64()?;
        let low = self.read_u64()?;

        let mut bytes: [u8;16] = [0u8; 16];
        bytes[..8].copy_from_slice(&high.to_be_bytes());
        bytes[8..].copy_from_slice(&low.to_be_bytes());

        match Uuid::from_slice(&bytes) {
            Ok(value) => {return Ok(value)},
            Err(_) => {return Err(SerializerError::SerializerMessage("Failed to read UUID".to_owned()))}
        }
    }


    pub fn read_fixed_bitset(&mut self, bits:usize) -> Result<Bytes, SerializerError> {
        return Ok(self.copy_to_bytes(bits.div_ceil(8))?)
    }

    
    pub fn read_bool(&mut self) -> Result<bool, SerializerError> {
        return Ok(self.read_u8()? != 0);
    }



    pub fn read_u64(&mut self) -> Result<u64, SerializerError> {
        if self.buffer.remaining() >= 8 {
            return Ok(self.buffer.get_u64());
        } else {
            Err(SerializerError::SerializerMessage("Failed to read u64".to_owned()))
        }
    }

    pub fn copy_to_bytes(&mut self,size:usize) -> Result<Bytes, SerializerError> {
        if self.buffer.len() >= size {
            return Ok(self.buffer.copy_to_bytes(size))
        } else {
            Err(SerializerError::SerializerMessage("Failed to copy bytes!".to_owned()))
        }
    }

    pub fn copy_to_slice(&mut self,dst:&mut [u8]) -> Result<(),SerializerError> {
        if self.buffer.remaining() >= dst.len() {
            self.buffer.copy_from_slice(dst);
            Ok(())
        } else {
            Err(SerializerError::SerializerMessage("Failed to copy from slice!".to_owned()))
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

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_var_int().unwrap(),-2147483648)

    }

    #[test]
    fn test_read_var_long() {
        let data:Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_var_long().unwrap(),-9223372036854775808)

    }

    #[test]
    fn test_read_string() {
        let data:Vec<u8> = vec![0x09, //String Size (9 bytes)
        0x6D, 0x69, 0x6E, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74]; //String Data

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ByteBuffer::new(data_buf);

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

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_uuid().unwrap(),Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
    }
}


impl Default for ByteBuffer {
    fn default() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }
 }