use grammers_client::types::Message;

pub fn constuct_mirror_msg(message: &Message) -> Option<String> {
    let message_text = message.text();
    let message_sender = message.sender()?;
    let sender_name = message_sender.name();
    let sender_username = message_sender.username().unwrap_or("NO_USERNAME");
    let formatted_message = format!(
        "name: {}\n\
         username: {}\n\
         ---------------\n\
         {}",
        sender_name, sender_username, message_text
    );

    Some(formatted_message)
}
