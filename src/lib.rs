pub mod connection;

#[test]
fn it_works() {
    use connection::Connection;

    let connection = Connection::connect("127.0.0.1:9481").unwrap();

    connection.request();

    connection.join().unwrap();
}
