use dotenv::dotenv;
use serde::de::value::Error;
use std::env;

#[derive(Debug)]
pub struct TelegramConfig {
    pub api_id: i32,
    pub api_hash: String,
    pub session_name: String,
    pub chat_ids: Vec<i64>,
}

impl TelegramConfig {
    pub fn new(prefix: &str) -> Self {
        dotenv().ok();

        Self {
            api_id: env::var(format!("{}_API_ID", prefix))
                .unwrap()
                .parse()
                .unwrap(),
            api_hash: env::var(format!("{}_API_HASH", prefix)).unwrap(),
            session_name: format!("{}.{}", prefix, "session"),
            chat_ids: Self::load_chat_ids(&format!("{}_CHAT_IDS", prefix)),
        }
    }

    fn load_chat_ids(env_var: &str) -> Vec<i64> {
        let comma_separated_ids = match env::var(env_var) {
            Ok(ids) => ids,
            Err(err) => {
                eprintln!("Could not load {} var: {}", env_var, err);
                panic!("Panic due to env var error");
            }
        };

        let mut chat_ids: Vec<i64> = Vec::new();
        for id in comma_separated_ids.split(", ") {
            let trimmed_id = id.trim();
            let parsed_id = match trimmed_id.parse::<i64>() {
                Ok(id) => id,
                Err(err) => {
                    eprintln!("Could not convert string to i64 for chat id: {}", id.trim());
                    panic!("Intentional panick due to error");
                }
            };
            chat_ids.push(parsed_id);
        }

        chat_ids
    }
}

pub fn load_telegram_configs() -> Result<Vec<TelegramConfig>, Error> {
    let prefixes: Vec<TelegramConfig> = env::var("PREFIXES")
        .unwrap()
        .to_uppercase()
        .split(',')
        .map(|s| TelegramConfig::new(s.trim()))
        .collect();

    if prefixes.is_empty() {
        panic!("PREFIXES env var not set properly!");
    }

    Ok(prefixes)
}
