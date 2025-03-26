use crate::{
    GroupchatsData, construct_alpha_call::construct_alpha_call,
    construct_call_message::construct_call_message, construct_mirror_message::constuct_mirror_msg,
    send_message::send_telegram_message,
};
use grammers_client::{Client, Update};
use std::error;

pub async fn listen_for_updates(
    client: Client,
    gc_data: &GroupchatsData,
) -> Result<(), Box<dyn error::Error>> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) if !message.outgoing() => {
                process_message(&client, gc_data, &message).await;
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}

async fn process_message(
    client: &Client,
    gc_data: &GroupchatsData,
    message: &grammers_client::types::Message,
) {
    let chat_id = message.chat().id();

    // Alpha call logic
    if let Some(alpha_message) = construct_alpha_call(message) {
        send_telegram_message(client, &gc_data.alpha_chat, &alpha_message).await;
    }

    // Mirror message logic
    if gc_data.mirror_from_chat.id() == chat_id {
        if let Some(formatted_message) = constuct_mirror_msg(message) {
            send_telegram_message(client, &gc_data.mirror_to_chat, &formatted_message).await;
        }
    }

    // Call message logic
    if let Some(call_message) = construct_call_message(message) {
        send_telegram_message(client, &gc_data.calls_chat, &call_message).await;
    }
}
