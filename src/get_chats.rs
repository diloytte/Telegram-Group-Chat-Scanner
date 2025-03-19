use grammers_client::Client;
use grammers_client::types::chat::Chat;

pub async fn get_all_chats(client: &Client) -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
    println!("Fetching all chats...");

    let mut chat_list: Vec<Chat> = Vec::new();
    let mut iter = client.iter_dialogs();

    //TODO: Unwrap
    let chats_total = iter.total().await.unwrap();

    let iter_count = chats_total;

    for _ in 0..iter_count {
        let next_dialog_result = iter.next().await;
        match next_dialog_result {
            Ok(next_dialog_option) => match next_dialog_option {
                Some(next_dialog) => {
                    // let dialog_id = next_dialog.chat.id();
                    // let dialog_name = next_dialog.chat.name();
                    // let dialog_username = next_dialog.chat.username().unwrap_or("NO_USERNAME");
                    // println!("Dialog ID: {}", dialog_id);
                    // println!("Dialog Name: {}", dialog_name);
                    // println!("Dialog Username: {}", dialog_username);
                    // println!("-----------------------------------------");
                    chat_list.push(next_dialog.chat);
                }
                None => println!("Dialog is None."),
            },
            Err(_) => {
                println!("Dialog invalid.");
                continue;
            }
        }
    }

    Ok(chat_list)
}
