use std::collections::{HashMap, HashSet};

use super::Message;

pub struct ClientHello {
    message_name: String,
    data: HashMap<String, String>,
    payload: Option<Vec<u8>>,
}

impl ClientHello {

    pub fn new() -> ClientHello {
        ClientHello {
            message_name: "ClientHello".to_string(),
            data: HashMap::new(),
            payload: None,
        }
    }

}

impl Message for ClientHello {

    fn get_message_name(&self) -> &str {
        &*self.message_name
    }

    fn set_field(&mut self, key: String, data: String) {
        self.data.insert(key, data);
    }

    fn get_all_fields(&self) -> &HashMap<String, String> {
        &self.data
    }

    fn set_payload(&mut self, data: Vec<u8>) {
        self.payload = Some(data);
    }

    fn get_payload(&mut self) -> Option<Vec<u8>> {
        self.payload.take()
    }
}
