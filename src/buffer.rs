use core::str;

use bytes::{Buf, BufMut, Bytes, BytesMut};

static SEGMENT_BITS:u8 = 0x7F;
static CONTINUE_BIT:u8 = 0x80;

 pub struct ByteBuffer {
    pub buffer: BytesMut
 }

 impl ByteBuffer {
    pub fn new(buffer:BytesMut) -> Self {
        Self{
            buffer
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        if self.buffer.has_remaining() {
            self.buffer.get_u8()
        } else {
            0
        }
    }


    pub fn read_var_int(&mut self) -> i32{
        let mut value:i32 = 0;
        let mut position:i32 = 0;
        let mut current_byte:u8 = 0;

        loop {
            current_byte = self.read_u8();

            value |= ((current_byte & SEGMENT_BITS as u8) as i32) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            
            position += 7;

            if position >= 32 {
                panic!("VarInt is too big");
            }
        }

        return value
    }

    pub fn read_var_long(&mut self) -> i64 {
        let mut value:i64 = 0;
        let mut position:i32 = 0;
        let mut current_byte:u8 = 0;

        loop {
            current_byte = self.read_u8();

            value |= ((current_byte & SEGMENT_BITS as u8) as i64) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            
            position += 7;

            if position >= 64 {
                panic!("VarInt is too big");
            }
        }

        return value;
    }
    
    pub fn read_string(&mut self,size:i32) -> String{
        let buf = self.read_var_int();
        println!("{:?}",buf);

        if buf > size {
            panic!("String is bigger than the max size!")
        }

        let data = self.copy_to_bytes(size as usize);

        if data.len() as i32 > size {
            panic!("String is bigger than the max size!")
        }
        
        return str::from_utf8(&data).expect("Failed to convert byte data to utf8 string!").to_string();
    }

    pub fn copy_to_bytes(&mut self,size:usize) -> Bytes {
        if self.buffer.len() >= size {
            return self.buffer.copy_to_bytes(size)
        } else {
            panic!("Failed to copy bytes!")
        }
    }

    pub fn copy_to_slice(&mut self,dst:&mut [u8]) {
        if self.buffer.remaining() >= dst.len() {
            self.buffer.copy_from_slice(dst);
        } else {
            panic!("Failed to copy from slice!")
        }
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_var_int() {
        let data:Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_var_int(),-2147483648)

    }

    #[test]
    fn test_read_var_long() {
        let data:Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_var_long(),-9223372036854775808)

    }

    #[test]
    fn test_read_string() {
        let data:Vec<u8> = vec![0x09, //String Size (9 bytes)
        0x6D, 0x69, 0x6E, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74]; //String Data

        let data_buf =  BytesMut::from(&data[..]);

        let mut buffer = ByteBuffer::new(data_buf);

        assert_eq!(buffer.read_string(9), "minecraft")

    }
}


impl Default for ByteBuffer {
    fn default() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }
 }