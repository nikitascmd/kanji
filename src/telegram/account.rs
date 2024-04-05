use super::TelegramConfig;
use grammers_client::{types::Dialog, Client, Config};
use grammers_session::Session;
use std::io::{stdin, stdout, Write};

/// Refers to Telegram Chats/Channels/Groups
pub struct TelegramGroup {
    telegram_id: String,
    name: String,
}

pub struct TelegramAccount {
    client: Client,
    tracked_groups: Vec<TelegramGroup>,
}

impl TelegramAccount {
    pub async fn new(config: TelegramConfig) -> Self {
        let client = match Self::init_client(&config).await {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Could not initialize Telegram Client: {}", err);
                panic!("Intentional panic due to Telegram Client initialization error");
            }
        };

        let tracked_groups = match Self::init_tracked_groups(&config.chat_ids[..], &client).await {
            Ok(groups) => groups,
            Err(err) => {
                eprintln!("Could not initialize tracked_groups: {}", err);
                panic!("Intentional panic due to tracked_groups initialization error");
            }
        };

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
            client.session().save_to_file(session_name)?; // Use `session_name` directly if it's of type `&str`. Otherwise, `.to_string()` or `.as_str()`
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
    ) -> Result<Vec<TelegramGroup>, Box<dyn std::error::Error>> {
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
        let tracked_groups: Vec<TelegramGroup> = dialogs_to_track
            .into_iter()
            .map(|dialog| {
                let chat = dialog.chat();
                TelegramGroup {
                    telegram_id: format!("{}", chat.id()),
                    name: chat.name().to_string(),
                }
            })
            .collect();

        Ok(tracked_groups)
    }
}
