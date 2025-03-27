use grammers_client::types::Message;

use crate::{
    extract_chat_data_from_message::extract_chat_data_from_message,
    token_address_extractors::extract_token_address_from_message_text,
};

use std::collections::HashMap;

pub fn construct_alpha_call(message: &Message) -> Option<String> {
    let chat_data = extract_chat_data_from_message(message);
    let chat_id = chat_data.2;

    //TODO: Not efficient, but currently only for testing purposes untill structured otherwise.
    let mut id_to_label: HashMap<i64, &str> = HashMap::new();
    id_to_label.insert(1058417098, "sap");
    id_to_label.insert(5365945926, "ay");
    id_to_label.insert(6682636432, "psy");
    // id_to_label.insert(7690346837, "dil");
    // id_to_label.insert(2361478254, "dil tr");
    id_to_label.insert(2049696512, "mil ge");

    if let Some(label) = id_to_label.get(&chat_id) {
        if let Some(token_address) = extract_token_address_from_message_text(message.text()) {
            return Some(format!("{}\n-------------------\n{}", label, token_address));
        }
    }

    None
}
