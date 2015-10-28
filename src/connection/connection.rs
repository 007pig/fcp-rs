use std::io;
use std::io::prelude::*;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::BufReader;
use std::sync::mpsc::*;

use std::thread;

use super::{EventCmd, EventResult};

pub struct Connection {
    stream: TcpStream,
    reader_thread: Option<thread::JoinHandle<()>>,
    tx_cmd: Sender<EventCmd>,
    rx_result: Receiver<EventResult>,
}

impl Connection {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Connection> {
        let stream = try!(TcpStream::connect(addr));

        let reader_stream = try!(stream.try_clone());

        // Channel to receive thread command from sender
        let (tx_cmd, rx_cmd) = channel::<EventCmd>();

        // Channel to send thread result to receiver
        let (tx_result, rx_result) = channel::<EventResult>();
        
        let tx_cmd = tx_cmd.clone();
        let reader_thread = thread::spawn(move || {

            println!("reading!");

            let reader = BufReader::new(reader_stream);

            for line in reader.lines() {
                let msg = line.unwrap();
                let mut result_msg = String::new();
                
                println!("buf: {:?}", &msg);
                result_msg.push_str(&*msg);

                if msg == "EndMessage" {
                    tx_result.send(EventResult::Message(msg));
                    result_msg = String::new();
                }
            }
            
        });

        Ok(Connection {
            stream: stream,
            reader_thread: Some(reader_thread),
            tx_cmd: tx_cmd,
            rx_result: rx_result,
        })
    }

    pub fn request(&self) {

        let mut stream = self.stream.try_clone().unwrap();
        
        let _ = stream.write(b"ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
        println!("stream sent!");

    }

    pub fn join(&mut self) -> thread::Result<()> {
        if let Some(reader_thread) = self.reader_thread.take() {
            reader_thread.join()
        }
        else {
            Err(Box::new("Thread already joined"))
        }
    }
}

