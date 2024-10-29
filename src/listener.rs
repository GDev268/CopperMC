use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::{sleep, Duration};
use bytes::{Buf, BytesMut};

mod server;
mod client;
mod packet;
mod buffer;
mod reader;

// Read a VarInt from a BytesMut buffer (same as before)
fn read_varint(buf: &mut BytesMut) -> Option<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read;
    loop {
        if num_read >= 5 {
            return None; // VarInt is too large
        }
        if buf.is_empty() {
            return None; // Not enough data
        }
        read = buf.get_u8() as i32;
        result |= (read & 0x7F) << (7 * num_read);
        num_read += 1;
        if (read & 0x80) == 0 {
            break;
        }
    }
    Some(result)
}

// A struct to hold the client connection
#[derive(Debug)]
struct Client {
    stream: TcpStream,
}

// A simple packet structure (using BytesMut for data)
#[derive(Debug)]
struct Packet {
    packet_id: i32,
    data: BytesMut,
}

impl Packet {
    // Parse the raw data from the stream into a Packet
    async fn from_stream(stream: &mut TcpStream) -> Option<Vec<Packet>> {
        let mut main_buffer = BytesMut::with_capacity(1024); // Using BytesMut for buffer

        let mut packets = Vec::new();
        match stream.read_buf(&mut main_buffer).await {
            Ok(size) => {
                if size > 0 {
                    // The first byte can be used as packet_id

                    while main_buffer.len() != 0 {
                        let length = read_varint(&mut main_buffer).unwrap();

                        let mut buffer = main_buffer.split_to(length as usize);

                        let packet_id = read_varint(&mut buffer).unwrap();

                        packets.push(Packet { packet_id, data: buffer });
                    }

                    Some(packets)
                } else {
                    None
                }
            }
            Err(_) => {
                // An error occurred, connection might be closed
                None
            }
        }
    }

    // Process the packet based on packet_id
    fn process(&self) {
        match self.packet_id {
            1 => println!("Packet Type: Login, Data: {:?}", self.data.iter().map(|b| *b as u8).collect::<Vec<u8>>()),
            2 => println!("Packet Type: Movement, Data: {:?}", self.data.iter().map(|b| *b as u8).collect::<Vec<u8>>()),
            3 => println!("Packet Type: Chat, Data: {:?}", self.data.iter().map(|b| *b as u8).collect::<Vec<u8>>()),
            _ => println!("Unknown Packet ID: {}, Data: {:?}", self.packet_id, self.data.iter().map(|b| *b as u8).collect::<Vec<u8>>()),
        }
    }
}

// The main server
struct Server {
    clients: HashMap<usize, Client>, // HashMap holding clients
    next_id: usize,                  // Unique ID for the next client
}

impl Server {
    // Create a new server
    fn new() -> Server {
        Server {
            clients: HashMap::new(),
            next_id: 0,
        }
    }

    // Add a new client to the server
    fn add_client(&mut self, stream: TcpStream) {
        self.clients.insert(self.next_id, Client { stream });
        println!("New client connected with ID: {}", self.next_id);
        self.next_id += 1;
    }

    // Process all clients asynchronously
    async fn process_clients(&mut self) {
        let mut disconnected_clients = Vec::new();

        for (id, client) in self.clients.iter_mut() {
            let stream = &mut client.stream;
            if let Some(packets) = Packet::from_stream(stream).await {
                println!("Received packets from client ID {}: {:?}", id, packets);

                for packet in packets {
                    packet.process();
                }
            } else {
                // Handle client disconnection
                if stream.write_u8(0).await.is_err() {
                    disconnected_clients.push(*id);
                }
            }
        }

        // Remove disconnected clients
        for id in disconnected_clients {
            self.clients.remove(&id);
            println!("Client ID {} disconnected", id);
        }
    }
}

async fn handle_client(stream: TcpStream, server: Arc<AsyncMutex<Server>>) {
    let mut locked_server = server.lock().await;
    locked_server.add_client(stream);
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.expect("Could not bind to address");
    let server = Arc::new(AsyncMutex::new(Server::new()));

    println!("Server started on 127.0.0.1:25565");
   
    let server_clone = Arc::clone(&server);
    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let server = Arc::clone(&server_clone);
                    tokio::spawn(handle_client(stream, server));
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    });

    // Main loop: process packets in a single thread
    loop {
        {
            let mut locked_server = server.lock().await;
            locked_server.process_clients().await;
        }
        sleep(Duration::from_millis(50)).await; // Sleep to prevent high CPU usage
    }
}
