use grammers_client::{Client, InvocationError};

#[allow(dead_code)]
pub async fn mark_all_chats_as_read(client: &Client) -> Result<(), InvocationError> {
    let mut dialog_iter = client.iter_dialogs();

    while let Ok(dialog_option) = dialog_iter.next().await {
        if let Some(dialog) = dialog_option {
            client.mark_as_read(dialog.chat()).await?;
        }
    }

    Ok(())
}
