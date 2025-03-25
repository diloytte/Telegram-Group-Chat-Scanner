mod find_chat;
mod get_chats;
mod construct_mirror_message;
mod construct_call_message;
mod new_message_listener;
mod print_dialog_data;
mod token_address_extractors;
mod send_message;
mod mark_all_chats_as_read;

use dotenv::dotenv;
use find_chat::find_chat;
use get_chats::get_all_chats;
use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;
use grammers_client::types::Chat;
use new_message_listener::listen_for_updates;
use std::env;
use tokio::fs;

pub struct GroupchatsData {
    pub mirror_from_chat: Chat,
    pub mirror_to_chat: Chat,
    pub calls_chat: Chat,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_id: i32 = env::var("API_ID")?.parse()?;
    let api_hash: String = env::var("API_HASH")?;
    let phone_number: String = env::var("PHONE_NUMBER")?;

    let session_file = "session.session";
    let session = if let Ok(data) = fs::read(session_file).await {
        Session::load(&data)?
    } else {
        Session::new()
    };

    let client = Client::connect(Config {
        session,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client.request_login_code(&phone_number).await?;

        println!("Enter the OTP code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                let password = env::var("PASSWORD")?;
                client.check_password(password_token, password).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    fs::write(session_file, session_data).await?;

    println!("Connected to Telegram!");

    let chats = get_all_chats(&client).await?;

    let calls_chat = find_chat(chats.clone(), "4733825356").await.unwrap().unwrap();

    let from_chat = find_chat(chats.clone(), "1981115066")
        .await
        .unwrap()
        .unwrap();
    let to_chat = find_chat(chats, "PP Forwards").await.unwrap().unwrap();

    let mirror_gc_data: GroupchatsData = GroupchatsData {
        mirror_from_chat:from_chat,
         mirror_to_chat:to_chat,
         calls_chat
        };

    let _ = listen_for_updates(client, &mirror_gc_data).await;

    Ok(())
}
