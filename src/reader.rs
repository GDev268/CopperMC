use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use std::str;
use uuid::Uuid;

#[derive(Debug)]
pub enum BufferError {
    DeserializerMessage(String),
    SerializerMesage(String),
}

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

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
    fn read_full_string(&mut self) -> Result<String, BufferError>;
    fn read_uuid(&mut self) -> Result<Uuid, BufferError>;
    fn read_option<T>(
        &mut self,
        function: impl Fn(&mut Self) -> Result<T, BufferError>,
    ) -> Option<T>;
    fn read_array<T>(&mut self, function: impl Fn(&mut Self) -> Result<T, BufferError>) -> Vec<T>;
    fn read_bitset(&mut self, len: usize) -> Result<Bytes, BufferError>;
    fn read_fixed_bitset(&mut self, len: usize) -> Result<Bytes, BufferError>;
    fn copy_buffer_to_bytes(&mut self, size: usize) -> Result<Bytes, BufferError>;
    fn copy_buffer_to_slice(&mut self, dst: &mut [u8]) -> Result<(), BufferError>;
}

impl ProtocolBufferReaderExt for BytesMut {
    fn read_bool(&mut self) -> Result<bool, BufferError> {
        return Ok(self.read_u8()? != 0);
    }

    fn read_i8(&mut self) -> Result<i8, BufferError> {
        if self.has_remaining() {
            Ok(self.get_i8())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read i8!".to_owned(),
            ))
        }
    }

    fn read_u8(&mut self) -> Result<u8, BufferError> {
        if self.has_remaining() {
            Ok(self.get_u8())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_i16(&mut self) -> Result<i16, BufferError> {
        if self.remaining() >= 2 {
            Ok(self.get_i16())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_u16(&mut self) -> Result<u16, BufferError> {
        if self.remaining() >= 2 {
            Ok(self.get_u16())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_i32(&mut self) -> Result<i32, BufferError> {
        if self.remaining() >= 4 {
            Ok(self.get_i32())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_u32(&mut self) -> Result<u32, BufferError> {
        if self.remaining() >= 4 {
            Ok(self.get_u32())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_i64(&mut self) -> Result<i64, BufferError> {
        if self.remaining() >= 8 {
            Ok(self.get_i64())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_u64(&mut self) -> Result<u64, BufferError> {
        if self.remaining() >= 8 {
            Ok(self.get_u64())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read u8!".to_owned(),
            ))
        }
    }

    fn read_f32(&mut self) -> Result<f32, BufferError> {
        if self.remaining() >= 4 {
            return Ok(self.get_f32());
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read f32".to_owned(),
            ))
        }
    }

    fn read_f64(&mut self) -> Result<f64, BufferError> {
        if self.remaining() >= 8 {
            return Ok(self.get_f64());
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to read f64".to_owned(),
            ))
        }
    }

    fn read_var_int(&mut self) -> Result<i32, BufferError> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_byte: u8 = 0;

        loop {
            current_byte = self.read_u8()?;

            value |= ((current_byte & SEGMENT_BITS as u8) as i32) << position;

            if current_byte as i32 & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(BufferError::DeserializerMessage(
                    "VarInt is too big".to_owned(),
                ));
            }
        }

        return Ok(value);
    }

    fn read_var_long(&mut self) -> Result<i64, BufferError> {
        let mut value: i64 = 0;
        let mut position: i32 = 0;
        let mut current_byte: u8 = 0;

        loop {
            current_byte = self.read_u8()?;

            value |= ((current_byte & SEGMENT_BITS as u8) as i64) << position;

            if current_byte as i32 & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 64 {
                return Err(BufferError::DeserializerMessage(
                    "VarLong is too big".to_owned(),
                ));
            }
        }

        return Ok(value);
    }

    fn read_full_string(&mut self) -> Result<String, BufferError> {
        self.read_string(i16::MAX.into())
    }

    fn read_string(&mut self, size: i32) -> Result<String, BufferError> {
        let buf = self.read_var_int()?;

        if buf > size {
            return Err(BufferError::DeserializerMessage(
                "String is bigger than the max size!".to_owned(),
            ));
        }

        let data = self.copy_buffer_to_bytes(buf as usize)?;

        if data.len() as i32 > size {
            panic!("String is bigger than the max size!")
        }

        return Ok(str::from_utf8(&data)
            .expect("Failed to convert byte data to utf8 string!")
            .to_string());
    }

    fn read_uuid(&mut self) -> Result<Uuid, BufferError> {
        let high = self.read_u64()?;
        let low = self.read_u64()?;

        let mut bytes: [u8; 16] = [0u8; 16];
        bytes[..8].copy_from_slice(&high.to_be_bytes());
        bytes[8..].copy_from_slice(&low.to_be_bytes());

        match Uuid::from_slice(&bytes) {
            Ok(value) => return Ok(value),
            Err(_) => {
                return Err(BufferError::DeserializerMessage(
                    "Failed to read UUID".to_owned(),
                ))
            }
        }
    }

    fn read_option<T>(
        &mut self,
        function: impl Fn(&mut Self) -> Result<T, BufferError>,
    ) -> Option<T> {
        if self.read_bool().unwrap() {
            Some(function(self).unwrap())
        } else {
            None
        }
    }

    fn read_array<T>(&mut self, function: impl Fn(&mut Self) -> Result<T, BufferError>) -> Vec<T> {
        let length = self.read_var_int().unwrap();

        let mut values = Vec::new();

        for _ in 0..length {
            values.push(function(self).unwrap());
        }

        values
    }

    fn read_bitset(&mut self, len: usize) -> Result<Bytes, BufferError> {
        self.copy_buffer_to_bytes(len)
    }

    fn read_fixed_bitset(&mut self, len: usize) -> Result<Bytes, BufferError> {
        self.copy_buffer_to_bytes(len.div_ceil(8))
    }

    fn copy_buffer_to_bytes(&mut self, size: usize) -> Result<Bytes, BufferError> {
        if self.len() >= size {
            return Ok(self.copy_to_bytes(size));
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to copy bytes!".to_owned(),
            ))
        }
    }

    fn copy_buffer_to_slice(&mut self, dst: &mut [u8]) -> Result<(), BufferError> {
        if self.remaining() >= dst.len() {
            self.copy_from_slice(dst);
            Ok(())
        } else {
            Err(BufferError::DeserializerMessage(
                "Failed to copy from slice!".to_owned(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_read_bool() {
        let mut data_buf = BytesMut::from(&[0x01][..]); // true
        assert_eq!(data_buf.read_bool().unwrap(), true);
    }

    #[test]
    fn test_read_i8() {
        let mut data_buf = BytesMut::from(&[0x7F][..]); // 127
        assert_eq!(data_buf.read_i8().unwrap(), 127);
    }

    #[test]
    fn test_read_u8() {
        let mut data_buf = BytesMut::from(&[0xFF][..]); // 255
        assert_eq!(data_buf.read_u8().unwrap(), 255);
    }

    #[test]
    fn test_read_i16() {
        let mut data_buf = BytesMut::from(&[0x01, 0x00][..]); // 256
        assert_eq!(data_buf.read_i16().unwrap(), 256);
    }

    #[test]
    fn test_read_u16() {
        let mut data_buf = BytesMut::from(&[0xFF, 0xFF][..]); // 65535
        assert_eq!(data_buf.read_u16().unwrap(), 65535);
    }

    #[test]
    fn test_read_i32() {
        let mut data_buf = BytesMut::from(&[0x00, 0x00, 0x01, 0x00][..]); // 256
        assert_eq!(data_buf.read_i32().unwrap(), 256);
    }

    #[test]
    fn test_read_u32() {
        let mut data_buf = BytesMut::from(&[0xFF, 0xFF, 0xFF, 0xFF][..]); // 4294967295
        assert_eq!(data_buf.read_u32().unwrap(), 4294967295);
    }

    #[test]
    fn test_read_i64() {
        let mut data_buf = BytesMut::from(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00][..]); // 256
        assert_eq!(data_buf.read_i64().unwrap(), 256);
    }

    #[test]
    fn test_read_u64() {
        let mut data_buf = BytesMut::from(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF][..]); // 18446744073709551615
        assert_eq!(data_buf.read_u64().unwrap(), 18446744073709551615);
    }

    #[test]
    fn test_read_f32() {
        let mut data_buf = BytesMut::from(&[0x3F, 0x80, 0x00, 0x00][..]); // 1.0 as f32
        assert_eq!(data_buf.read_f32().unwrap(), 1.0);
    }

    #[test]
    fn test_read_f64() {
        let mut data_buf = BytesMut::from(&[0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]); // 1.0 as f64
        assert_eq!(data_buf.read_f64().unwrap(), 1.0);
    }

    #[test]
    fn test_read_var_int() {
        let mut data_buf = BytesMut::from(&[0x80, 0x80, 0x80, 0x80, 0x08][..]); // -2147483648 as VarInt
        assert_eq!(data_buf.read_var_int().unwrap(), -2147483648);
    }

    #[test]
    fn test_read_var_long() {
        let data: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];

        let mut buffer = BytesMut::from(&data[..]);

        assert_eq!(buffer.read_var_long().unwrap(), -9223372036854775808);
    }

    #[test]
    fn test_read_string() {
        let mut data_buf = BytesMut::from(&[0x05, b'H', b'e', b'l', b'l', b'o'][..]); // "Hello" string with length prefix
        assert_eq!(data_buf.read_string(5).unwrap(), "Hello");
    }

    #[test]
    fn test_read_full_string() {
        let mut data_buf = BytesMut::from(&[0x05, b'H', b'e', b'l', b'l', b'o'][..]); // "Hello" string with length prefix
        assert_eq!(data_buf.read_full_string().unwrap(), "Hello");
    }

    #[test]
    fn test_read_uuid() {
        let mut data_buf = BytesMut::from(
            &[
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                0xDE, 0xF1,
            ][..],
        ); // Some UUID representation
        let expected_uuid = Uuid::from_bytes([
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
            0xDE, 0xF1,
        ]);

        assert_eq!(data_buf.read_uuid().unwrap(), expected_uuid);
    }

    #[test]
    fn test_read_bitset() {
        let mut data_buf = BytesMut::from(&[0b10101010, 0b11110000][..]); // Example bitset data
        assert_eq!(
            data_buf.read_bitset(2).unwrap(),
            Bytes::from(&[0b10101010, 0b11110000][..])
        );
    }
}
