use bytes::BytesMut;
use crate::reader::ProtocolBufferReaderExt;

pub struct Packet {
    pub id: i32,
    pub buffer: BytesMut
}