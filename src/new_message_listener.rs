use std::error;

use grammers_client::Client;

use crate::{
    GroupchatsData, construct_call_message::construct_call_message,
    construct_mirror_message::constuct_mirror_msg, send_message::send_telegram_message,
};

pub async fn listen_for_updates(
    client: Client,
    gc_data: &GroupchatsData,
) -> Result<(), Box<dyn error::Error>> {
    loop {
        match client.next_update().await {
            Ok(update) => match update {
                grammers_client::Update::NewMessage(message) => {
                    if message.outgoing() {continue;}
                    let from_chat = &gc_data.mirror_from_chat;
                    let chat_id = message.chat().id();

                    if from_chat.id() == chat_id {
                        let formated_message_option = constuct_mirror_msg(&message);

                        let to_chat = &gc_data.mirror_to_chat;

                        if let Some(formatted_message) = formated_message_option {
                            send_telegram_message(&client, to_chat, &formatted_message).await
                        }
                    }

                    let call_message_option = construct_call_message(&message);

                    if let Some(call_message) = call_message_option {
                        println!("{}", call_message);
                        send_telegram_message(&client, &gc_data.calls_chat, &call_message).await
                    }
                }
                _ => {}
            },
            Err(e) => {
                println!("Error in listen_for_updates: {}", e);
                continue;
            }
        }
    }
}
