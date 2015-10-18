use std::net::ToSocketAddrs;

use super::Connection;


enum ConnectionStatus {
    Closed,
    Connected(Connection),
    Disconnected,
}

pub struct ConnectionManager {
    connections: Vec<ConnectionStatus>,
}

impl ConnectionManager {
    pub fn create_connection<A: ToSocketAddrs>(addr: A) {
        let connection = Connection.connect(addr)
    }
    
}
