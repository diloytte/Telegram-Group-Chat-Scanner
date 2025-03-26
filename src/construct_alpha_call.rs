use grammers_client::types::Message;

use crate::{
    extract_chat_data::extract_chat_data_from_message,
    token_address_extractors::extract_token_address_from_message_text,
};

pub fn construct_alpha_call(message: &Message) -> Option<String> {
    let chat_data = extract_chat_data_from_message(message);

    let mut token_address_final: Option<String> = None;

    //1058417098 sap
    //5365945926 ay
    //6682636432 psy
    //7690346837 dil
    //2361478254 dil tr
    //2049696512 mil ge

    let accepted_ids: Vec<i64> = vec![
        1058417098, 5365945926, 6682636432, 7690346837, 2361478254, 2049696512,
    ];
    let chat_id = chat_data.2;
    if accepted_ids.contains(&chat_id) {
        println!("alpha caller detected");
        if let Some(token_address) = extract_token_address_from_message_text(message.text()) {
            token_address_final = Some(token_address);
        }
    }

    token_address_final
}
