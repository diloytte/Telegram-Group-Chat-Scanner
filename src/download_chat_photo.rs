use grammers_client::Client;


#[allow(dead_code)]
pub async fn download_profile_photo(client: &Client, username: &str, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let chat_option = client.resolve_username(username).await?;
    if chat_option.is_none() {
        return Err(format!("Chat with username {} not found", username).into());
    }
    let chat = chat_option.unwrap();  
    let chat_downloadable_photo = chat.photo_downloadable(false).unwrap();
    client.download_media(&chat_downloadable_photo, save_path).await?;
    Ok(())
}
