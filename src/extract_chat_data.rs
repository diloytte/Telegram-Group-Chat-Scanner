use grammers_client::types::Chat;
use grammers_client::types::Message;

pub fn extract_chat_data_from_message(message: &Message) -> (String, String, i64) {
    let from_chat = message.chat();
    let mut name = String::new();
    let mut username = String::new();
    let mut id = i64::default();

    if let Chat::User(user) = from_chat {
        name = user.full_name();
        username = user
            .username()
            .map_or("NO_USERNAME".to_string(), String::from);
        id = user.id();
    } else if let Chat::Channel(channel) = from_chat {
        name = channel.title().to_string();
        id = channel.id();
    } else if let Some(sender) = message.sender() {
        username = sender
            .username()
            .map_or("NO_USERNAME".to_string(), String::from);
        name = sender.name().to_string();
        id = sender.id();
    }

    (name, username, id)
}
