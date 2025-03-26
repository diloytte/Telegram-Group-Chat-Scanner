use crate::extract_chat_data::extract_chat_data_from_message;
use crate::token_address_extractors::extract_token_address_from_message_text;
use grammers_client::types::Message;

pub fn construct_call_message(message: &Message) -> Option<String> {
    let _ = message.chat();
    let token_address_option = extract_token_address_from_message_text(message.text());

    if let Some(token_address) = token_address_option {
        let (sender_name, sender_username, sender_id) = extract_chat_data_from_message(message);

        let accepted_ids: Vec<i64> = vec![
            6856832897, //Drn
            1046947851, //Rkt
            7377078796, //TNFLB
            5346291682, //De
            1160171995, //M
            //////////
            1058417098, 5365945926, 6682636432, 7690346837, 2361478254, 2049696512,
        ];

        // 7178305557 phanes
        // 6362041475 pirb
        // 7774196337 phanes gold

        // let ignored_ids: Vec<i64> = vec![7178305557, 6362041475,7774196337];

        if !accepted_ids.contains(&sender_id) {
            return None;
        }

        let final_message = format!(
            "Name: {}\nUsername: {}\nID: {}\n--------------\n{}",
            sender_name, sender_username, sender_id, token_address
        );
        return Some(final_message);
    }

    None
}
