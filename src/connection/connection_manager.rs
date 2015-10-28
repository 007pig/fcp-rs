use std::net::ToSocketAddrs;
use std::collections::HashMap;
use std::thread;
use std::io;
use std::sync::mpsc::*;

use super::{Connection, EventCmd, EventResult};

pub enum ConnectionStatus {
    Closed,
    Connected(Connection),
    Disconnected,
}

pub struct ConnectionManager {
    connections: HashMap<&'static str, ConnectionStatus>,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }
    
    pub fn create_connection<A: ToSocketAddrs>(&mut self, key: &'static str, addr: &A) {
        let mut connection = Connection::connect(addr);

        while connection.is_err() {
            // Sleep some time and try again
            thread::sleep_ms(5000);
            
            connection = Connection::connect(addr);
        }

        self.connections.insert(key, ConnectionStatus::Connected(connection.unwrap()));
    }

    pub fn get_connection_mut(&mut self, key: &str) -> Option<&mut Connection> {
        if let Some(connection_status) = self.connections.get_mut(key) {
            match connection_status {
                &mut ConnectionStatus::Connected(ref mut connection) => {
                    return Some(connection);
                },
                _ => return None,
            }
        }
        else {
            None
        }
    }

    pub fn get_connection_ref(&self, key: &str) -> Option<&Connection> {
        if let Some(connection_status) = self.connections.get(key) {
            match connection_status {
                &ConnectionStatus::Connected(ref connection) => {
                    return Some(connection);
                },
                _ => return None,
            }
        }
        else {
            None
        }
    }

    pub fn join_connection(&mut self, key: &str) -> thread::Result<()> {
        if let Some(connection) = self.get_connection_mut(key) {
            connection.join()
        }
        else {
            Err(Box::new("Connection doesn't exist"))
        }
    }

    pub fn request_str(&mut self, key: &str, str_data: &str) -> io::Result<()>{
        if let Some(connection) = self.get_connection_mut(key) {
            connection.request_str(str_data)
        }
        else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Connection doesn't exist"))
        }
        
    }

    pub fn get_rx_result(&self, key: &str) -> io::Result<&Receiver<EventResult>> {        
        if let Some(connection) = self.get_connection_ref(key) {
            Ok(connection.get_rx_result())
        }
        else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Connection doesn't exist"))
        }
    }
}
