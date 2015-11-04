use std::collections::{HashMap, HashSet};

pub trait Message<'a> : Send{

    fn get_message_name(&self) -> &str;

    fn set_field(&mut self, key: &'a str, data: &'a str);

    fn get_all_fields(&'a self) -> &'a HashMap<&str, &str>;

    fn set_payload(&mut self, data: &'a [u8]);

    fn get_payload(&self) -> Option<&[u8]>;

    fn to_vec(&'a self) -> Vec<u8> {

        let mut message_str = String::new();

        message_str.push_str(self.get_message_name());
        message_str.push_str("\n");

        for (key, value) in self.get_all_fields() {
            message_str.push_str(&format!("{}={}", key, value));
            message_str.push_str("\n");
        }

        if let Some(payload) = self.get_payload() {            
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

}

