use buffer::ByteBuffer;
use bytes::{buf, Bytes, BytesMut};

mod buffer;

fn main() {
    let data: Vec<u8> = vec![0x07, 0x47, 0x61, 0x62, 0x72, 0x69, 0x65, 0x6c ];

    let data_buf =  BytesMut::from(&data[..]);

    let mut buffer = ByteBuffer::new(data_buf);


    //println!("{:?}",buffer.read_var_int() == 2147483647 );    
    //println!("{:?}",buffer.read_u8() == 255 );    
    println!("{:?}",buffer.read_string(7));
    
    println!("Hello, world!");
}
