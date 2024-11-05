use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::packet::Packet;
use crate::reader::ProtocolBufferReaderExt;

pub struct Client {
    pub stream: TcpStream,
    packet_queue: Vec<Packet>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            packet_queue: Vec::new(),
        }
    }

    //Rreturns an bool depending if the client disconnects
    pub async fn get_incoming_packets(&mut self) -> bool {
        let mut main_buffer = BytesMut::with_capacity(1024);
        let mut count: u16 = 0;

        match self.stream.read_buf(&mut main_buffer).await {
            Ok(size) => {
                if size > 0 {
                    while main_buffer.len() != 0 {
                        let length = main_buffer.read_var_int().unwrap();
    
                        let mut buffer = main_buffer.split_to(length as usize);
    
                        let packet_id = buffer.read_var_int().unwrap();
    
                        self.packet_queue.push(Packet { id: packet_id, buffer });
                        count += 1;
                    }

                    println!("Received {:?} incoming packets from {:?}",count,self.stream.local_addr().unwrap());

                    return true;
                }
            },
            Err(_) => {
                self.test_client_disconnection().await;
            }
        }

        return false;
    }

    pub async fn test_client_disconnection(&mut self) -> bool {
        let mut buffer = [0u8; 1];
        match self.stream.peek(&mut buffer).await {
            Ok(0) => true,
            Ok(_) => false,
            Err(_) => true,
        }
    }

    pub fn process_packets(&mut self) {
        for i in 0..self.packet_queue.len() {
            let packet = self.packet_queue.get_mut(i).unwrap();
            
            //let protocol_version = packet.buffer.read_var_int().unwrap();

            //let server_address = packet.buffer.read_string(255).unwrap();

            //let server_port = packet.buffer.read_u16().unwrap();

            //let next_state = packet.buffer.read_var_int().unwrap();

            println!("Received packet ID: {:?} with content: {:?}",packet.id,packet.buffer);
            //println!("Handshake info: \nProtocol Version: {:?}\nServer address: {:?}\nServer Port: {:?}\nNext State: {:?}",protocol_version,server_address,server_port,next_state);
        }

        self.packet_queue.clear();
    }
}
