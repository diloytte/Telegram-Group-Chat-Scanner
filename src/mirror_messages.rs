use grammers_client::{types::Chat, Client};

pub async fn mirror_msgs(client:&Client,mirroring_chat:&Chat,listening_chat:&Chat)->Result<(),Box<dyn std::error::Error>>{
    let mirroring_chat_id = mirroring_chat.id();
    loop {
        match client.next_update().await{
            Ok(update) => {
                match update {
                    grammers_client::Update::NewMessage(message) => {
                        let chat = message.chat();
                        let chat_id = chat.id();
                        if mirroring_chat_id != chat_id {
                            continue;
                        }
                        let message_text = message.text();
                        let message_sender = match message.sender() {
                            Some(sender) => sender,
                            None => continue,
                        };
                        let sender_name = message_sender.name();
                        let sender_username = message_sender.username().unwrap_or("NO_USERNAME");
                        let formatted_message = format!(
                            "name: {}\n\
                             username: {}\n\
                             ---------------\n\
                             {}",
                            sender_name, sender_username, message_text
                        );
                        match client.send_message(listening_chat, formatted_message).await {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error sending message: {}\n Error: {}",message_text,e)
                            },
                        }
                    },
                    _ => {
                    },
                }
            },
            Err(e) => {
                println!("Error in mirror_msgs: {}",e);
                continue;
            },
        }
    }
}