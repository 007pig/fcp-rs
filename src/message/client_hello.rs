use std::collections::{HashMap, HashSet};

use super::Message;

pub struct ClientHello<'a> {
    message_name: &'a str,
    data: HashMap<&'a str, &'a str>,
    payload: Option<&'a [u8]>,
}

impl<'a> ClientHello<'a> {

    pub fn new() -> ClientHello<'a> {
        ClientHello {
            message_name: "ClientHello",
            data: HashMap::new(),
            payload: None,
        }
    }

}

impl<'a> Message<'a> for ClientHello<'a> {

    fn get_message_name(&self) -> &str {
        self.message_name
    }

    fn set_field(&mut self, key: &'a str, data: &'a str) {
        self.data.insert(key, data);
    }

    fn get_all_fields(&'a self) -> &'a HashMap<&str, &str> {
        &self.data
    }

    fn set_payload(&mut self, data: &'a [u8]) {
        self.payload = Some(data);
    }

    fn get_payload(&self) -> Option<&[u8]> {
        self.payload
    }
}
