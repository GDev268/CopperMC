use bytes::BytesMut;

use crate::buffer::ProtocolBuffer;

pub struct Packet {
    pub id: i32,
    pub buffer: BytesMut
}