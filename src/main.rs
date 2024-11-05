use rand::Rng;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::io::{self, AsyncReadExt};
use std::sync::Arc;
use std::collections::HashMap;
use bytes::BytesMut;
use std::net::SocketAddr;
use uuid::Uuid; // Use Uuid for unique client IDs

fn generate_random_uuid() -> Uuid {
    let mut rng = rand::thread_rng();
    // Generate 16 random bytes
    let random_bytes: [u8; 16] = rng.gen();

    // Create a UUID with the random bytes
    Uuid::from_bytes(random_bytes)
}

// Server struct to hold connected clients
struct Server {
    clients: Mutex<HashMap<Uuid, (SocketAddr, BytesMut)>>,
}

impl Server {
    fn new() -> Self {
        Server {
            clients: Mutex::new(HashMap::new()),
        }
    }

    async fn add_client(&self, id: Uuid, addr: SocketAddr, buffer: BytesMut) {
        let mut clients = self.clients.lock().await;
        clients.insert(id, (addr, buffer));
    }

    async fn remove_client(&self, id: &Uuid) {
        let mut clients = self.clients.lock().await;
        clients.remove(id);
        println!("Client {} removed", id);
    }

    async fn print_packets(&self) {
        let mut clients = self.clients.lock().await;
        for (id, (addr, buffer)) in clients.iter_mut() {
            if buffer.len() > 0 {
                println!("Client {} ({}): {:?}", id, addr, buffer);
                buffer.clear();
            }

        }
    }

    async fn print_all_clients(&self) {
        // This method is called to print all clients in a synchronized manner
        self.print_packets().await;
    }
}

async fn handle_client(server: Arc<Server>, mut stream: TcpStream, addr: SocketAddr) {
    let mut buffer = BytesMut::with_capacity(1024); // Adjust buffer size as needed
    let client_id = generate_random_uuid();
    
    // Add client initially
    server.add_client(client_id, addr, buffer.clone()).await;

    loop {
        let mut temp_buffer = [0u8; 512]; // Temporary buffer for reading
        match stream.read(&mut temp_buffer).await {
            Ok(0) => {
                // Connection was closed by the client
                println!("Client {} disconnected", addr);
                break;
            }
            Ok(bytes_read) => {
                // Extend the buffer with the read data
                buffer.extend_from_slice(&temp_buffer[..bytes_read]);
                // Add/Update client in the server
                server.add_client(client_id, addr, buffer.clone()).await;

                // Here we can call a method to print all clients if needed
                // However, we will keep it outside to avoid clutter in this function
            }
            Err(e) => {
                eprintln!("Error reading from client {}: {}", addr, e);
                break;
            }
        }
    }

    // Remove client when done
    server.remove_client(&client_id).await;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    println!("Server running on 127.0.0.1:25565");

    let server = Arc::new(Server::new());

    // Spawn a separate task to periodically print clients
    {
        let server = Arc::clone(&server);
        tokio::spawn(async move {
            loop {
                server.print_all_clients().await; // Print all clients every 5 seconds
            }
        });
    }

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let server = Arc::clone(&server);
                tokio::spawn(async move {
                    handle_client(server, stream, addr).await;
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
