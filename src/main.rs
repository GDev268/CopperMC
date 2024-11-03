use bytes::{Buf, BytesMut};
use server::Server;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::{sleep, Duration};

mod client;
mod packet;
mod reader;
mod writer;
mod server;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565")
        .await
        .expect("Could not bind to address");
    let server = Arc::new(AsyncMutex::new(Server::new()));

    println!("Server started on 127.0.0.1:25565");

    let server_clone = Arc::clone(&server);
    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let server = Arc::clone(&server_clone);
                    tokio::spawn(async move {
                        let mut locked_server = server.lock().await;
                        locked_server.add_client(stream);
                    });
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    });

    // Spawn a task for processing clients independently.
    let server_clone = Arc::clone(&server);
    tokio::spawn(async move {
        loop {
            let mut locked_server = server_clone.lock().await;
            locked_server.process_clients().await;
            // Add a short delay if needed to avoid tight looping.
            sleep(Duration::from_millis(10)).await;
        }
    });

    // Main loop that isn't blocked by client processing.
    loop {
        println!("L");
        sleep(Duration::from_millis(100)).await; // Adjust timing as needed for your application.
    }
}
