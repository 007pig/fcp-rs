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
        
        let reader_thread = thread::spawn(move || {

            println!("reading!");

            let mut reader = BufReader::new(reader_stream);

            let mut result_msg = String::new();
            let mut line = String::new();
            
            loop {
                reader.read_line(&mut line).unwrap();

                // Concatenate Line to result message
                result_msg.push_str(&line);

                if line == "EndMessage\n" {
                    tx_result.send(EventResult::Message(result_msg.clone())).unwrap();
                    result_msg.clear();
                }

                // Clear line buffer
                line.clear();

                match rx_cmd.try_recv() {
                    Ok(event_cmd) => {
                        match event_cmd {
                            EventCmd::Close => break,
                        }
                    },
                    Err(TryRecvError::Disconnected) => break,
                    Err(_) => (),
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

    pub fn request_str(&self, str_data: &str) -> io::Result<()> {

        let mut stream = try!(self.stream.try_clone());
        
        //let _ = stream.write(b"ClientHello\nName=My Freenet Client\nExpectedVersion=2.0\nEndMessage\n");
        let _ = try!(stream.write(str_data.as_bytes()));

        println!("stream sent!");

        Ok(())

    }

    pub fn get_rx_result(&self) -> &Receiver<EventResult> {
        &self.rx_result
    }

    pub fn get_tx_cmd(&self) -> &Sender<EventCmd> {
        &self.tx_cmd
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

