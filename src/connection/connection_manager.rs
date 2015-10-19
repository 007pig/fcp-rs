use std::net::ToSocketAddrs;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use super::Connection;


enum ConnectionStatus {
    Closed,
    Connected(Connection),
    Disconnected,
}

pub struct ConnectionManager {
    connections: HashMap<&str, ConnectionStatus>,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }
    
    pub fn create_connection<A: ToSocketAddrs>(&mut self, key: &str, addr: A) {
        let connection = Connection.connect(addr);

        while connection.is_err() {
            // Sleep some time and try again
            thread::sleep_ms(5000);
            
            connection = Connection.connect(attr);
        }

        self.connections.insert(key, connection);
    }

    pub fn get_connection(&self, key: &str) -> ConnectionStatus {
        self.connections.get(key)
    }
}
