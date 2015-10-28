pub mod connection;

#[test]
fn it_works() {
    use connection::{ConnectionManager, ConnectionStatus};
    
    let mut connection_manager = ConnectionManager::new();

    let key = "connection_1";
    
    connection_manager.create_connection(key, &"127.0.0.1:9481");

    connection_manager.request_str(key, "ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n").unwrap();

    connection_manager.join_connection(key).unwrap();

    // if let Some(connection_status) = connection_manager.get_connection_mut("connection_1") {
    //     match connection_status {
    //         &mut ConnectionStatus::Connected(ref mut connection) => {
    //             connection.request_str("ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
    //             connection.join().unwrap();
    //         },
                
    //         _ => unimplemented!(), 
    //     }
    // }

}
