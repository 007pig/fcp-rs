pub mod connection;

#[test]
fn it_works() {
    use connection::{ConnectionManager, ConnectionStatus};
    use std::thread;

    let mut connection_manager = ConnectionManager::new();

    connection_manager.create_connection("connection_1", &"127.0.0.1:9481");

    if let Some(connection_status) = connection_manager.get_connection("connection_1") {
        match connection_status {
            &ConnectionStatus::Connected(ref connection) => 
                connection.request(),
                
            _ => unimplemented!(), 
        }
    }

    thread::park();

}
