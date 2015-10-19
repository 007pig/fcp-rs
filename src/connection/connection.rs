use std::io;
use std::io::prelude::*;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::BufReader;

use std::thread;

pub struct Connection {
    stream: TcpStream,
    reader_thread: thread::JoinHandle<()>,
}

impl Connection {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Connection> {
        let stream = try!(TcpStream::connect(addr));

        let reader_stream = try!(stream.try_clone());
        let reader_thread = thread::spawn(move || {

            println!("reading!");

            let reader = BufReader::new(reader_stream);

            for line in reader.lines() {
                let msg = line.unwrap();
                
                println!("buf: {:?}", &msg);

                if msg == "EndMessage" {
                    break;
                }
            }
            
        });

        Ok(Connection {
            stream: stream,
            reader_thread: reader_thread
        })
    }

    pub fn request(&self) {

        let mut stream = self.stream.try_clone().unwrap();
        
        let _ = stream.write(b"ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
        println!("stream sent!");

    }

    pub fn join(self) -> thread::Result<()> {
        try!(self.reader_thread.join());

        Ok(())
    }
}
