use grammers_client::types::chat::Chat;

pub async fn find_chat<'a>(chats:&'a Vec<Chat>,identifier: &str) -> Result<Option<&'a Chat>, Box<dyn std::error::Error>> {
    for chat in chats {
        if chat.id().to_string() == identifier || chat.name() == identifier  {
            return Ok(Some(chat));
        }
    }
    
    Ok(None)
}
