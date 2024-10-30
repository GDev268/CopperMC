use bytes::{Buf, BytesMut};
use server::Server;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::{sleep, Duration};

mod client;
mod packet;
mod reader;
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

    loop {
        let mut locked_server = server.lock().await;
        locked_server.process_clients().await;

        sleep(Duration::from_millis(50)).await; // Sleep to prevent high CPU usage
    }
}