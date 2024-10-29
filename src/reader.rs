use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;
use std::str;
use std::error::Error;

#[derive(Debug)]
pub enum BufferError {
    DeserializerMessage(String),
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub trait ProtocolBufferReaderExt {
    fn read_bool(&mut self) -> Result<bool, BufferError>;
    fn read_i8(&mut self) -> Result<i8, BufferError>;
    fn read_u8(&mut self) -> Result<u8, BufferError>;
    fn read_i16(&mut self) -> Result<i16, BufferError>;
    fn read_u16(&mut self) -> Result<u16, BufferError>;
    fn read_i32(&mut self) -> Result<i32, BufferError>;
    fn read_u32(&mut self) -> Result<u32, BufferError>;
    fn read_i64(&mut self) -> Result<i64, BufferError>;
    fn read_u64(&mut self) -> Result<u64, BufferError>;
    fn read_f32(&mut self) -> Result<f32, BufferError>;
    fn read_f64(&mut self) -> Result<f64, BufferError>;
    fn read_var_int(&mut self) -> Result<i32, BufferError>;
    fn read_var_long(&mut self) -> Result<i64, BufferError>;
    fn read_string(&mut self, size: i32) -> Result<String, BufferError>;
    fn read_full_string(&mut self) -> Result<String,BufferError>;
    fn read_uuid(&mut self) -> Result<Uuid, BufferError>;
    fn copy_buffer_to_bytes(&mut self, size: usize) -> Result<Bytes, BufferError>;
    fn copy_buffer_to_slice(&mut self,dst:&mut [u8]) -> Result<(),BufferError>;
}

impl ProtocolBufferReaderExt for BytesMut {
    fn read_bool(&mut self) -> Result<bool, BufferError> {
        return Ok(self.read_u8()? != 0);
    }

    fn read_i8(&mut self) -> Result<i8, BufferError> {
        if self.has_remaining() {
            Ok(self.get_i8())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read i8!".to_owned()))
        }
    }

    fn read_u8(&mut self) -> Result<u8, BufferError> {
        if self.has_remaining() {
            Ok(self.get_u8())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_i16(&mut self) -> Result<i16, BufferError> {
        if self.remaining() >= 2 {
            Ok(self.get_i16())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_u16(&mut self) -> Result<u16, BufferError> {
        if self.remaining() >= 2 {
            Ok(self.get_u16())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_i32(&mut self) -> Result<i32, BufferError> {
        if self.remaining() >= 4 {
            Ok(self.get_i32())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_u32(&mut self) -> Result<u32, BufferError> {
        if self.remaining() >= 4 {
            Ok(self.get_u32())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_i64(&mut self) -> Result<i64, BufferError> {
        if self.remaining() >= 8 {
            Ok(self.get_i64())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_u64(&mut self) -> Result<u64, BufferError> {
        if self.remaining() >= 8 {
            Ok(self.get_u64())
        } else {
            Err(BufferError::DeserializerMessage("Failed to read u8!".to_owned()))
        }
    }

    fn read_f32(&mut self) -> Result<f32, BufferError> {
        if self.remaining() >= 4 {
            return Ok(self.get_f32());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read f32".to_owned()))
        }
    }

    fn read_f64(&mut self) -> Result<f64, BufferError> {
        if self.remaining() >= 8 {
            return Ok(self.get_f64());
        } else {
            Err(BufferError::DeserializerMessage("Failed to read f64".to_owned()))
        }
    }

    fn read_var_int(&mut self) -> Result<i32, BufferError> {
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

    fn read_var_long(&mut self) -> Result<i64, BufferError> {
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

    fn read_full_string(&mut self) -> Result<String,BufferError> {
        self.read_string(i16::MAX.into())
    }

    fn read_string(&mut self, size: i32) -> Result<String, BufferError> {
        let buf = self.read_var_int()?;

        if buf > size {
            return Err(BufferError::DeserializerMessage("String is bigger than the max size!".to_owned()));
        }

        let data = self.copy_buffer_to_bytes(size as usize)?;

        if data.len() as i32 > size {
            panic!("String is bigger than the max size!")
        }
        
        return Ok(str::from_utf8(&data).expect("Failed to convert byte data to utf8 string!").to_string());
    }

    fn read_uuid(&mut self) -> Result<Uuid, BufferError> {
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

    fn copy_buffer_to_bytes(&mut self, size: usize) -> Result<Bytes, BufferError> {
        if self.len() >= size {
            return Ok(self.copy_to_bytes(size))
        } else {
            Err(BufferError::DeserializerMessage("Failed to copy bytes!".to_owned()))
        }
    }
    
    fn copy_buffer_to_slice(&mut self,dst:&mut [u8]) -> Result<(),BufferError> {
        if self.remaining() >= dst.len() {
            self.copy_from_slice(dst);
            Ok(())
        } else {
            Err(BufferError::DeserializerMessage("Failed to copy from slice!".to_owned()))
        }
    }
}