use grammers_client::types::Chat;
use grammers_client::types::Message;

pub fn extract_chat_data_from_message(message: &Message) -> (String, String, i64, u8) {
    let from_chat = message.chat();
    let mut name = String::new();
    let mut username = String::new();
    let mut id = i64::default();
    let mut chat_type = 0; // Default to User (0)

    if let Chat::User(user) = from_chat {
        name = user.full_name();
        username = user
            .username()
            .map_or("NO_USERNAME".to_string(), String::from);
        id = user.id();
        chat_type = 0; // User
    } else if let Chat::Channel(channel) = from_chat {
        name = channel.title().to_string();
        id = channel.id();
        chat_type = 1; // Channel
    } else if let Some(sender) = message.sender() {
        username = sender
            .username()
            .map_or("NO_USERNAME".to_string(), String::from);
        name = sender.name().to_string();
        id = sender.id();
        chat_type = 0; // Assume it's a User if no other type is detected
    }

    (name, username, id, chat_type)
}
