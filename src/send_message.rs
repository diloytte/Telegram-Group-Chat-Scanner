use grammers_client::Client;
use grammers_client::types::chat::Chat;

pub async fn send_telegram_message(client: &Client, to_chat:&Chat, message: &str) {
    match client.send_message(to_chat, message).await {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending message: {}\n Error: {}", message, e);
        }
    }
}
