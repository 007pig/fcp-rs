use std::net::ToSocketAddrs;
use std::collections::HashMap;
use std::thread;

use super::Connection;


pub enum ConnectionStatus {
    Closed,
    Connected(Connection),
    Disconnected,
}

pub struct ConnectionManager<'a> {
    connections: HashMap<&'a str, ConnectionStatus>,
}

impl<'a> ConnectionManager<'a> {
    pub fn new() -> ConnectionManager<'a> {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }
    
    pub fn create_connection<A: ToSocketAddrs>(&mut self, key: &'a str, addr: &A) {
        let mut connection = Connection::connect(addr);

        while connection.is_err() {
            // Sleep some time and try again
            thread::sleep_ms(5000);
            
            connection = Connection::connect(addr);
        }

        self.connections.insert(key, ConnectionStatus::Connected(connection.unwrap()));
    }

    pub fn get_connection(&self, key: &str) -> Option<&ConnectionStatus> {
        self.connections.get(key)
    }
}
