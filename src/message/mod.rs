pub mod message;
pub mod client_hello;
pub mod node_hello;

pub use self::message::Message;
pub use self::client_hello::ClientHello;
pub use self::node_hello::NodeHello;

pub fn parse_message(message_str: &str, payload: Option<&[u8]>) -> Result<Box<Message>, String> {

    let mut message : Option<Box<Message>> = None;

    // // Generate message object
    for line in message_str.lines() {
        message = message_factory(line);
        if message.is_some() {
            break;
        }
    }

    // // Unwrap from Option
    let mut message = message.unwrap();

    // // Continue getting lines from the iterator
    for line in message_str.lines() {

        //println!("{:?}", line);
        
        if line.to_string() == "EndMessage" {
            // Time to return
            return Ok(message);
        }
        // Set fields
        let v: Vec<&str> = line.split('=').collect();
        if v.len() == 2 {
            message.set_field(v[0].to_string(), v[1].to_string());
        }
    }
    
    Err("Unable to parse message".to_string())

}

fn message_factory(message_name: &str) -> Option<Box<Message>> {

    //println!("{}", message_name);
    
    match message_name {
        "NodeHello" => Some(Box::new(NodeHello::new())),
        _ => None,
    }
    
}
