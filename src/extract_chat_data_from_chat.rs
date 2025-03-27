use std::{
    fs::File,
    io::{BufWriter, Write},
    os::unix::ffi::OsStrExt,
};

use grammers_client::types::Chat;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatData {
    pub name: String,
    pub username: String,
    pub id: i64,
    pub chat_type: u8,
}

pub fn extract_chat_data_from_chat(chat: &Chat) -> ChatData {
    let mut name = String::new();
    let mut username = String::new();
    let mut id = i64::default();
    let mut chat_type = 0;

    if let Chat::User(user) = chat {
        name = user.full_name();
        username = user
            .username()
            .map_or("NO_USERNAME".to_string(), String::from);
        id = user.id();
        chat_type = 0;
    } else if let Chat::Channel(channel) = chat {
        name = channel.title().to_string();
        id = channel.id();
        chat_type = 1;
    } else if let Chat::Group(group) = chat {
        name = group.title().to_string();
        id = group.id();
        chat_type = 2;
    }

    return ChatData {
        name,
        username,
        id,
        chat_type,
    };
}

pub fn extract_chats_data_from_chats(chats: Vec<Chat>) -> Vec<ChatData> {
    let mut data_array = Vec::new();

    for chat in chats {
        data_array.push(extract_chat_data_from_chat(&chat));
    }

    data_array
}

pub fn save_json_to_file<T>(json_data: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let json_string = to_string_pretty(json_data)?;
    writer.write_all(json_string.as_bytes())?;
    writer.flush()?;
    Ok(())
}
