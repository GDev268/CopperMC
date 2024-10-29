use std::collections::VecDeque;

use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::packet::Packet;
use crate::reader::ProtocolBufferReaderExt;

struct Client {
    stream: TcpStream,
    packet_queue: VecDeque<Packet>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            packet_queue: VecDeque::new(),
        }
    }

    pub async fn get_incoming_packets(&mut self) {
        let mut main_buffer = BytesMut::with_capacity(1024);
        let mut count: u16 = 0;

        if let Ok(size) = self.stream.read_buf(&mut main_buffer).await {
            if size > 0 {
                while main_buffer.len() != 0 {
                    let length = main_buffer.read_var_int().unwrap();

                    let mut buffer = main_buffer.split_to(length as usize);

                    let packet_id = main_buffer.read_var_int().unwrap();

                    self.packet_queue.push_back(Packet { id: packet_id, buffer });
                    count += 1;
                }
            }
        }

        let addr = match self.stream.peer_addr() {
            Ok(addr) => Ok(addr.to_string()), // Convert the socket address to a string
            Err(e) => Err(e),
        }.unwrap();


        println!("Received {:?} incoming packets from {:?}",count,addr)
    }
}
