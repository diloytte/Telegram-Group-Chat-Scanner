use grammers_client::{Client, InvocationError};

use crate::GroupchatsData;

pub async fn forward_redacted_systems_bot_message(
    client: &Client,
    gc_data: &GroupchatsData,
    message: &grammers_client::types::Message,
) -> Result<(),InvocationError> {
    let chat_id = message.chat().id();
    let redacted_systems_id = &gc_data.redacted_system_bot_chat.id();
    let redacted_forwards = &gc_data.redacted_forwards_chat;

    if chat_id == *redacted_systems_id {
        client.forward_messages(redacted_forwards, &[message.id()], &gc_data.redacted_system_bot_chat).await?;
    }
    
    Ok(())
}