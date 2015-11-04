use std::io;
use std::io::prelude::*;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::BufReader;
use std::sync::mpsc::*;
use std::sync::Arc;

use std::str::FromStr;

use std::thread;

use super::{EventCmd, EventResult};
use ::message::{parse_message, Message};

pub struct Connection<'a> {
    stream: TcpStream,
    reader_thread: Option<thread::JoinHandle<()>>,
    tx_cmd: Sender<EventCmd>,
    rx_result: Receiver<EventResult<'a>>,
}

impl<'a> Connection<'a> {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Connection<'static>> {
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

            let mut data_length:u64 = 0;
            let mut payload:Vec<u8> = Vec::new();
            
            loop {
                reader.read_line(&mut line).unwrap();

                // Concatenate Line to result message
                result_msg.push_str(&line);

                if line == "EndMessage\n" {

                    let message = parse_message(&result_msg, None).unwrap();
                    
                    tx_result.send(EventResult::Message(message)).unwrap();
                    // Cleanup
                    result_msg.clear();
                    data_length = 0;
                }

                if line.starts_with("DataLength") {
                    // Try to get DataLength
                    let v: Vec<&str> = line.split('=').collect();
                    if v.len() == 2 {
                        data_length = u64::from_str(v[1].trim()).unwrap();
                    }
                }

                if line == "Data\n" && data_length > 0 {
                    // Read payload buf
                    let mut tmp_vec = Vec::with_capacity(data_length as usize);
                    let mut buffer = &mut tmp_vec[..];
                    reader.read(&mut buffer).unwrap();

                    payload.extend(buffer.iter().cloned());
                    
                    // Cleanup
                    result_msg.clear();
                    data_length = 0;
                    payload.clear();
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

    pub fn get_rx_result(&self) -> &Receiver<EventResult<'a>> {
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

