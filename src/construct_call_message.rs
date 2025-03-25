use grammers_client::types::Message;
use grammers_client::types::chat::Chat;

use crate::token_address_extractors::extract_token_address_from_message_text;

pub fn construct_call_message(message: &Message) -> Option<String> {
    let from_chat = message.chat();
    let token_address_option = extract_token_address_from_message_text(message.text());

    let mut sender_name = String::new();
    let mut sender_username =String::new();
    let mut sender_id = i64::default();

    if let Some(token_address) = token_address_option {
        if let Chat::User(user) = from_chat {
            // println!("USER");
            // println!("{:?}",&user);
            sender_name = user.full_name();
            sender_username = match user.username() {
                Some(username) => String::from(username),
                None => String::from("NO_USERNAME"),
            };
            sender_id = user.id();
        } else if let Chat::Channel(channel) = from_chat {
            // println!("CHANNEL");
            sender_name = channel.title().to_string();
            sender_id = channel.id();
        }else {
            // println!("SENDER");
            if let Some(sender) = message.sender() {
                sender_username =  match sender.username() {
                    Some(username) => String::from(username),
                    None => String::from("NO_USERNAME"),
                };
                sender_name = sender.name().to_string();
                sender_id = sender.id();
            } else {
                println!("No sender information found for this message.");
            }    
        }

        // println!("{:?}", &sender_name);
        // println!("{:?}", &sender_username);
        // println!("{:?}", &sender_id);
        // println!("---------------------------");
        let final_message = format!("Name: {}\nUsername: {}\nID: {}\n--------------\n{}",
            sender_name, sender_username, sender_id, token_address
        );
        return Some(final_message);
    }


    return None;
}
