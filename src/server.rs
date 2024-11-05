use std::{collections::HashMap, net::SocketAddr};

use tokio::net::TcpStream;
use uuid::Uuid;

use crate::client::Client;

struct GlobalConfiguration {
    pub enable_packet_encryption: bool,
    pub enable_packet_compression: bool,
}

impl Default for GlobalConfiguration {
    fn default() -> Self {
        Self {
            enable_packet_encryption: false,
            enable_packet_compression: false,
        }
    }
}

pub struct Server {
    pub clients: HashMap<SocketAddr, Client>,
    pub global_config: GlobalConfiguration,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            global_config: Default::default(),
        }
    }

    pub fn add_client(&mut self,stream: TcpStream) {
        println!("New client connected with Address: {:?}", &stream.local_addr().unwrap());
        self.clients.insert(stream.local_addr().unwrap(), Client::new(stream));
    }

    pub async fn process_clients(&mut self) {
        let mut disconnected_clients = Vec::new();

        for (addr , client) in self.clients.iter_mut() {
            if !client.get_incoming_packets().await {
                disconnected_clients.push(*addr);
                continue;
            };

            client.process_packets();
        }
    }
}
