use grammers_client::{types::Dialog, Client, Config};
use grammers_session::Session;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use super::TelegramConfig;

/// Refers to Telegram Chats/Channels/Groups
#[derive(Clone)]
pub struct TelegramGroup {
    pub telegram_id: i64, // gramme.rs has telegram group ids as i64
    pub name: String,
}

#[derive(Clone)]
pub struct TelegramAccount {
    pub client: Client,
    pub tracked_groups: HashMap<i64, TelegramGroup>,
}

impl TelegramAccount {
    pub async fn new(config: TelegramConfig) -> Self {
        let client = Self::init_client(&config).await.unwrap();
        let tracked_groups = Self::init_tracked_groups(&config.chat_ids, &client)
            .await
            .unwrap();

        Self {
            client,
            tracked_groups,
        }
    }

    async fn init_client(config: &TelegramConfig) -> Result<Client, Box<dyn std::error::Error>> {
        let TelegramConfig {
            api_id,
            api_hash,
            session_name,
            ..
        } = config;

        let session = Session::load_file_or_create(session_name)?;
        let client = Client::connect(Config {
            session,
            api_id: *api_id,
            api_hash: api_hash.clone(),
            params: Default::default(),
        })
        .await?;

        if !client.is_authorized().await? {
            let phone_number = Self::prompt("Enter your phone number: ");
            let request_result = client.request_login_code(&phone_number).await?;
            let code = Self::prompt("Enter the code you received: ");

            client.sign_in(&request_result, &code).await?;
            client.session().save_to_file(session_name)?;
        }

        Ok(client)
    }

    fn prompt(message: &str) -> String {
        let mut input = String::new();
        print!("{}", message);
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    pub async fn init_tracked_groups(
        chat_ids: &[i64],
        client: &Client,
    ) -> Result<HashMap<i64, TelegramGroup>, Box<dyn std::error::Error>> {
        let mut dialogs_iterator = client.iter_dialogs();
        let mut dialogs_to_track: Vec<Dialog> = Vec::new();

        // 1. Iterate over dialogs and collect those that match the chat_ids
        // No need for performance efficiency since it's only run on start and `chat_ids` will be small size
        while let Some(dialog) = dialogs_iterator.next().await? {
            if chat_ids.contains(&dialog.chat().id()) {
                dialogs_to_track.push(dialog);
            }
        }

        // 2. Transform the tracked dialogs into `TelegramGroup` instances
        let tracked_groups: HashMap<i64, TelegramGroup> = dialogs_to_track
            .into_iter()
            .map(|dialog| {
                let chat = dialog.chat();
                let chat_id = chat.id();
                let telegram_group = TelegramGroup {
                    telegram_id: chat_id,
                    name: chat.name().to_string(),
                };
                (chat_id, telegram_group)
            })
            .collect();

        Ok(tracked_groups)
    }
}
