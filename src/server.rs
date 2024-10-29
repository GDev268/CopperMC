use std::collections::HashMap;

use uuid::Uuid;

use crate::Client;

struct GlobalConfiguration {
    pub enable_packet_encryption: bool,
    pub enable_packet_compression: bool,
}

struct Server {
    clients: HashMap<Uuid,Client>,
    global_config: GlobalConfiguration
}

