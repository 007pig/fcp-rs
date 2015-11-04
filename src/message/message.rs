use std::collections::{HashMap, HashSet};

trait Message {

    fn get_message_name(&self) -> &str;

    fn set_field(&mut self, key: &str, data: &str);

    fn get_all_fields(&self) -> HashMap<&str, &str>;

    fn set_payload(&mut self, data: &[u8]);

    fn get_payload(&self) -> Option<&[u8]>

    fn as_bytes(&self) -> &[u8] {

        let mut message_str = String::new();

        message_str.push_str(self.get_message_name);
        message_str.push_str("\n");

        for (key, value) in &self.get_all_fields() {
            message_str.push_str(format!("{}={}", key, value));
            message_str.push_str("\n");
        }

        let mut result_buf;
        
        if let Some(payload) = self.get_payload() {            
            message_str.push_str(format!("DataLength={}", payload.len()));
            message_str.push_str("\n");
            
            message_str.push_str("Data\n");
            result_buf = message_str.into_bytes();

            result_buf.append(payload)
        }
        else {
            message_str.push_str("EndMessage\n");
            message_str.as_bytes()
        }
        
    }

}
