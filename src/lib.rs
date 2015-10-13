use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;


#[test]
fn it_works() {
    let mut stream = TcpStream::connect("127.0.0.1:9481").unwrap();

    let _ = stream.write(b"ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
    println!("stream sent!");
    

    //let mut buf = String::new();

    println!("reading!");

    let mut reader = BufReader::new(stream);
    
    //let n = reader.read_line(&mut buf);

    for line in reader.lines() {
        let msg = line.unwrap();
        
        println!("buf: {:?}", &msg);

        if msg == "EndMessage" {
            break;
        }
    }
}
