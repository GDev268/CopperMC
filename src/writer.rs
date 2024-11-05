use crate::reader::BufferError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use std::str;
use uuid::Uuid;

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub trait ProtocolBufferWriterExt {
    fn write_bool(&mut self, value: &bool);
    fn write_i8(&mut self, value: &i8);
    fn write_u8(&mut self, value: &u8);
    fn write_i16(&mut self, value: &i16);
    fn write_u16(&mut self, value: &u16);
    fn write_i32(&mut self, value: &i32);
    fn write_u32(&mut self, value: &u32);
    fn write_i64(&mut self, value: &i64);
    fn write_u64(&mut self, value: &u64);
    fn write_f32(&mut self, value: &f32);
    fn write_f64(&mut self, value: &f64);
    fn write_string(&mut self, value: &str, max_size: i32);
    fn write_full_string(&mut self, value: &str);
    fn write_var_int(&mut self, value: &i32);
    fn write_var_long(&mut self, value: &i64);
    fn write_uuid(&mut self, value: &Uuid);
    fn write_option<T>(&mut self, value: Option<T>, function: impl Fn(&mut Self, &T));
    fn write_array<T>(&mut self, value: Vec<T>, function: impl Fn(&mut Self, &T));
    fn write_bitset(&mut self, value: &[i64]);
}

impl ProtocolBufferWriterExt for BytesMut {
    fn write_bool(&mut self, value: &bool) {
        if *value {
            self.put_u8(1);
        } else {
            self.put_u8(0);
        }
    }

    fn write_i8(&mut self, value: &i8) {
        self.put_i8(*value);
    }

    fn write_u8(&mut self, value: &u8) {
        self.put_u8(*value);
    }

    fn write_i16(&mut self, value: &i16) {
        self.put_i16(*value);
    }

    fn write_u16(&mut self, value: &u16) {
        self.put_u16(*value);
    }

    fn write_i32(&mut self, value: &i32) {
        self.put_i32(*value);
    }

    fn write_u32(&mut self, value: &u32) {
        self.put_u32(*value);
    }

    fn write_i64(&mut self, value: &i64) {
        self.put_i64(*value);
    }

    fn write_u64(&mut self, value: &u64) {
        self.put_u64(*value);
    }

    fn write_f32(&mut self, value: &f32) {
        self.put_f32(*value);
    }

    fn write_f64(&mut self, value: &f64) {
        self.put_f64(*value);
    }

    fn write_string(&mut self, value: &str, max_size: i32) {
        if value.len() as i32 > max_size {
            panic!("String is too big!")
        }

        self.write_var_int(&(value.len() as i32));
        self.put(value.as_bytes());
    }

    fn write_full_string(&mut self, value: &str) {
        self.write_string(value, i16::MAX.into());
    }

    fn write_var_int(&mut self, mut value: &i32) {
        let mut final_value = *value;
        loop {
            if (final_value & !SEGMENT_BITS) == 0 {
                self.write_u8(&(final_value as u8));
                return;
            }

            self.write_u8(&(((final_value & SEGMENT_BITS) | CONTINUE_BIT) as u8));

            final_value >>= 7;
        }
    }

    fn write_var_long(&mut self, value: &i64) {
        let mut final_value = *value;
        loop {
            if (final_value & !SEGMENT_BITS as i64) == 0 {
                self.write_u8(&(final_value as u8));
                return;
            }

            self.write_u8(&(((final_value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8));

            final_value >>= 7;
        }
    }

    fn write_uuid(&mut self, value: &Uuid) {
        let bytes = value.as_bytes();

        let low = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);

        let high = u64::from_be_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);

        self.write_u64(&low);
        self.write_u64(&high);
    }

    fn write_option<T>(&mut self, value: Option<T>, function: impl Fn(&mut Self, &T)) {
        if value.is_none() {
            self.write_bool(&false);
        } else {
            self.write_bool(&true);
            function(self, &value.unwrap())
        }
    }

    fn write_array<T>(&mut self, value: Vec<T>, function: impl Fn(&mut Self, &T)) {
        for i in 0..value.len() {
            function(self, &value[i])
        }
    }

    fn write_bitset(&mut self, value: &[i64]) {
        self.write_var_int(&value.len().try_into().unwrap());

        for byte in value {
            println!("{:X}",*byte);
            self.write_i64(byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    use rand::RngCore;
    use uuid::{Uuid, Builder, Variant, Version};
    use crate::reader::ProtocolBufferReaderExt;

    #[cfg(test)]
    mod tests {
        use super::*;
        use bytes::BytesMut;
        use uuid::Uuid;

        #[test]
        fn test_write_bool() {
            let value = true;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_bool(&value);

            // Read the value back
            let result = buffer.read_bool().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_i8() {
            let value: i8 = -123;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_i8(&value);

            // Read the value back
            let result = buffer.read_i8().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_u8() {
            let value: u8 = 200;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_u8(&value);

            // Read the value back
            let result = buffer.read_u8().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_i16() {
            let value: i16 = -32000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_i16(&value);

            // Read the value back
            let result = buffer.read_i16().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_u16() {
            let value: u16 = 65000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_u16(&value);

            // Read the value back
            let result = buffer.read_u16().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_i32() {
            let value: i32 = -2000000000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_i32(&value);

            // Read the value back
            let result = buffer.read_i32().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_u32() {
            let value: u32 = 4000000000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_u32(&value);

            // Read the value back
            let result = buffer.read_u32().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_i64() {
            let value: i64 = -9000000000000000000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_i64(&value);

            // Read the value back
            let result = buffer.read_i64().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_u64() {
            let value: u64 = 18000000000000000000;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_u64(&value);

            // Read the value back
            let result = buffer.read_u64().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_f32() {
            let value: f32 = 3.14159;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_f32(&value);

            // Read the value back
            let result = buffer.read_f32().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_f64() {
            let value: f64 = 2.718281828459045;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_f64(&value);

            // Read the value back
            let result = buffer.read_f64().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_string() {
            let value = "Hello, world!";
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_string(&value, 256);

            // Read the value back
            let result = buffer.read_string(256).unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_full_string() {
            let value = "Full test string!";
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_full_string(&value);

            // Read the value back
            let result = buffer.read_full_string().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_var_int() {
            let value: i32 = 123456;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_var_int(&value);

            // Read the value back
            let result = buffer.read_var_int().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_var_long() {
            let value: i64 = 1234567890123456789;
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_var_long(&value);

            // Read the value back
            let result = buffer.read_var_long().unwrap();
            assert_eq!(result, value);
        }

        #[test]
        fn test_write_uuid() {
            let mut bytes = [0u8; 16];

            rand::thread_rng().fill_bytes(&mut bytes);
        
            let mut binding = Builder::from_bytes(bytes);
            let value = binding
                .set_variant(Variant::RFC4122)
                .set_version(Version::Random).as_uuid();


            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_uuid(value);

            // Read the value back
            let result = buffer.read_uuid().unwrap();
            assert_eq!(result, *value);
        }

        #[test]
        fn test_write_bitset() {
            let value: Vec<i64> = vec![0b10101010, 0b11110000];
            let mut buffer = BytesMut::new();

            // Write the value
            buffer.write_bitset(&value);

            // Read the value back
            let result = buffer.read_fixed_bitset(2).unwrap();
            assert_eq!(result, Bytes::from(&[0b10101010, 0b11110000][..]));
        }
    }
}
