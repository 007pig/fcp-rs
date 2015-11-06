use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum MessageType {
    ClientHello,
    NodeHello,
}

pub struct Message {    
    data: HashMap<String, String>,
    payload: Option<Vec<u8>>,
    message_type: MessageType,
}

impl Message {

    pub fn new(message_type: MessageType) -> Message {
        Message {
            data: HashMap::new(),
            payload: None,
            message_type: message_type,
        }
    }

    pub fn get_message_name(&self) -> String {
        format!("{:?}", self.message_type)
    }

    pub fn set_field(&mut self, key: String, data: String) {
        self.data.insert(key, data);
    }

    pub fn get_all_fields(&self) -> &HashMap<String, String> {
        &self.data
    }

    pub fn set_payload(&mut self, data: Vec<u8>) {
        self.payload = Some(data);
    }

    pub fn get_payload(&mut self) -> Option<Vec<u8>> {
        self.payload.take()
    }

    pub fn to_vec(&mut self) -> Vec<u8> {

        let mut message_str = String::new();

        message_str.push_str(&*self.get_message_name());
        message_str.push_str("\n");

        for (key, value) in &self.data {
            message_str.push_str(&format!("{}={}", key, value));
            message_str.push_str("\n");
        }

        if let Some(ref payload) = self.payload {            
            message_str.push_str(&format!("DataLength={}", payload.len()));
            message_str.push_str("\n");
            
            message_str.push_str("Data\n");
       
            let mut result_buf = message_str.into_bytes();

            result_buf.extend(payload.iter().cloned());

            result_buf
        }
        else {
            message_str.push_str("EndMessage\n");
            message_str.into_bytes()
        }
        
    }

    pub fn to_string(&mut self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }

}

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
        "NodeHello" => Some(Box::new(Message::new(MessageType::NodeHello))),
        _ => None,
    }
    
}
