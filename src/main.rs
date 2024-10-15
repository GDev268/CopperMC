use buffer::ByteBuffer;
use bytes::{buf, Bytes, BytesMut};
use serde::de::Visitor;

mod buffer;

fn main() {
    let data: Vec<u8> = vec![
        0x55, 0x0e, 0x84, 0x00, // 4 bytes
        0xe2, 0x9b, // 2 bytes
        0x41, 0xd4, // 2 bytes
        0xa7, 0x16, // 2 bytes
        0x44, 0x66, 0x55, 0x44, 0x00, 0x00 // 6 bytes
    ];

    let data_buf =  BytesMut::from(&data[..]);

    let mut buffer = ByteBuffer::new(data_buf);


    //println!("{:?}",buffer.read_var_int() == 2147483647 );    
    //println!("{:?}",buffer.read_u8() == 255 );    
    println!("{:?}",buffer.read_uuid().unwrap());

    
    println!("Hello, world!");
}
