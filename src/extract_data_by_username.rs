use grammers_client::{Client, InvocationError};

#[allow(dead_code)]
pub async fn extract_data_by_username(
    client: &Client,
    username: &str,
) -> Result<(i64, String), InvocationError> {
    let chat_option = client.resolve_username(username).await?;

    let mut username_id = i64::default();
    let mut name = String::default();

    if let Some(chat) = chat_option {
        username_id = chat.id();
        name = String::from(chat.name());
    }

    Ok((username_id, name))
}
