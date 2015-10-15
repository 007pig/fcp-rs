use std::io;
use std::io::prelude::*;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::BufReader;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Connection> {
        let stream = try!(TcpStream::connect(addr));

        Ok(Connection {
            stream: stream
        })
    }

    pub fn request(&self) {

        let mut stream = self.stream.try_clone().unwrap();
        
        let _ = stream.write(b"ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
        println!("stream sent!");
        

        //let mut buf = String::new();

        println!("reading!");

        let reader = BufReader::new(stream);
        
        //let n = reader.read_line(&mut buf);

        for line in reader.lines() {
            let msg = line.unwrap();
            
            println!("buf: {:?}", &msg);

            if msg == "EndMessage" {
                break;
            }
        }
    }
}

