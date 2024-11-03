use crate::reader::BufferError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use std::str;
use uuid::Uuid;

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub trait ProtocolBufferWriterExt {
    fn write_bool(&mut self, value: bool);
    fn write_i8(&mut self, value: i8);
    fn write_u8(&mut self, value: u8);
    fn write_i16(&mut self, value: i16);
    fn write_u16(&mut self, value: u16);
    fn write_i32(&mut self, value: i32);
    fn write_u32(&mut self, value: u32);
    fn write_i64(&mut self, value: i64);
    fn write_u64(&mut self, value: u64);
    fn write_f32(&mut self, value: f32);
    fn write_f64(&mut self, value: f64);
    fn write_string(&mut self, value: String, max_size: i32);
    fn write_full_string(&mut self, value: String);
    fn write_var_int(&mut self, value: i32);
    fn write_var_long(&mut self, value: i64);
    fn write_uuid(&mut self, value: Uuid);
    fn write_bitset(&mut self, value: &[i64]);
}

impl ProtocolBufferWriterExt for BytesMut {
    fn write_bool(&mut self, value: bool) {
        if value {
            self.put_u8(1);
        } else {
            self.put_u8(0);
        }
    }

    fn write_i8(&mut self, value: i8) {
        self.put_i8(value);
    }

    fn write_u8(&mut self, value: u8) {
        self.put_u8(value);
    }

    fn write_i16(&mut self, value: i16) {
        self.put_i16(value);
    }

    fn write_u16(&mut self, value: u16) {
        self.put_u16(value);
    }

    fn write_i32(&mut self, value: i32) {
        self.put_i32(value);
    }

    fn write_u32(&mut self, value: u32) {
        self.put_u32(value);
    }

    fn write_i64(&mut self, value: i64) {
        self.put_i64(value);
    }

    fn write_u64(&mut self, value: u64) {
        self.put_u64(value);
    }

    fn write_f32(&mut self, value: f32) {
        self.put_f32(value);
    }

    fn write_f64(&mut self, value: f64) {
        self.put_f64(value);
    }

    fn write_string(&mut self, value: String, max_size: i32) {
        if value.len() as i32 > max_size {
            panic!("String is too big!")
        }

        self.write_var_int(value.len() as i32);
        self.put(value.as_bytes());
    }

    fn write_full_string(&mut self, value: String) {
        self.write_string(value, i16::MAX.into());
    }

    fn write_var_int(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.write_u8(value as u8);
                return;
            }

            self.write_u8(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            value >>= 7;
        }
    }

    fn write_var_long(&mut self, mut value: i64) {
        loop {
            if (value & !SEGMENT_BITS as i64) == 0 {
                self.write_u8(value as u8);
                return;
            }

            self.write_u8(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8);

            value >>= 7;
        }
    }

    fn write_uuid(&mut self, value: Uuid) {
        let bytes = value.as_bytes();

        let low = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);

        let high = u64::from_be_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);

        self.write_u64(low);
        self.write_u64(high);
    }

    fn write_bitset(&mut self, value: &[i64]) {
        todo!()
    }
}
