use std::net::ToSocketAddrs;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};

use super::Connection;


pub enum ConnectionStatus {
    Closed,
    Connected(Connection),
    Disconnected,
}

pub struct ConnectionManager<'a> {
    connections: HashMap<&'a str, ConnectionStatus>,
    senders: HashMap<&'a str, Sender>,
    receiver: &Receiver
}

impl<'a> ConnectionManager<'a> {
    pub fn new(receiver: &Receiver) -> ConnectionManager<'a> {
        ConnectionManager {
            connections: HashMap::new(),
            senders: HashMap::new(),
            receiver: receiver,
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

    pub fn get_connection_mut(&mut self, key: &str) -> Option<&mut ConnectionStatus> {
        self.connections.get_mut(key)
    }

    pub fn get_connection(&self, key: &str) -> Option<&ConnectionStatus> {
        self.connections.get(key)
    }

    pub fn join_connection(&mut self, key: &str) -> thread::Result<()> {
        if let Some(connection_status) = self.connections.get_mut(key) {
            match connection_status {
                &mut ConnectionStatus::Connected(ref mut connection) => {
                    return connection.join()
                },
                _ => unimplemented!()
            }
        }
        else {
            Err(Box::new("Connection doesn't exist"))
        }
    }
}
