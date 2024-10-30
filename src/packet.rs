use bytes::BytesMut;
use crate::reader::ProtocolBufferReaderExt;

#[derive(Debug)]
pub struct Packet {
    pub id: i32,
    pub buffer: BytesMut
}