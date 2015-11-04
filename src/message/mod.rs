pub mod message;
pub mod client_hello;

pub use self::message::Message;
pub use self::client_hello::ClientHello;


pub fn parse_message(message_str: &str, payload: &[u8]) -> Message {
    
}
