use crate::buffer::ByteBuffer;


#[derive(Debug,Clone)]
pub enum SerializerError{
    SerializerMessage(String)
}


struct Deserializer<'a> {
    buffer: &'a ByteBuffer
}

