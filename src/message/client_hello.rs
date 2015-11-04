use std::collections::{HashMap, HashSet};

use super::message::Message;

pub struct ClientHello {
    message_name: &str,
    data: HashMap<&str, &str>,
    payload: Option<&[u8]>,
}

impl Message for ClientHello {

    pub fn new() -> ClientHello {
        ClientHello {
            message_name: "ClientHello",
            data: HashMap::new(),
            payload: None,
        }
    }

    pub fn get_message_name(&self) -> &str {
        self.message_name
    }

    pub fn set_field(&mut self, key: &str, data: &str) {
        self.data.insert(key, data);
    }

    pub fn get_all_fields(&self) -> HashMap<&str, &str> {
        self.data
    }

    pub fn set_payload(&mut self, data: &[u8]) {
        self.payload = Some(data);
    }

    pub fn get_payload(&self) -> Option<&[u8]> {
        self.payload
    }
}
