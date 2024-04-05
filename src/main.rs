use dotenv::dotenv;
use telegram::{load_telegram_configs, TelegramAccount};
use tokio;

mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let telegram_configs = load_telegram_configs().unwrap();
    let mut telegram_accounts = Vec::new();

    for config in telegram_configs.into_iter() {
        let account = TelegramAccount::new(config).await;
        telegram_accounts.push(account);
    }

    Ok(())
}
