use grammers_client::Update;
use std::any::type_name;
use telegram::{DefaultParser, TelegramAccount, TelegramConfig};

mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("lol");

    let telegram_config = TelegramConfig::new("KEKI");
    let telegram_account = TelegramAccount::new(telegram_config).await;
    let telegram_parser = DefaultParser::new(telegram_account.clone());

    // let chat =
    // let me = telegram_account.client.iter_messages().await?;
    // println!("{:?}", me);
    // println!("{:?}", type_of(&me));

    // while let Some(update) = telegram_account.client.next_update().await? {
    //     match update {
    //         Update::NewMessage(message) if !message.outgoing() => {
    //             if let Some(parse_result) = telegram_parser.parse(message).await? {
    //                 println!("parse_result: {:?}", parse_result);
    //             } else {
    //                 continue;
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    /*
    let solana_rpc_url = env::var("SOLANA_RPC_URL").unwrap();
    let client = RpcClient::new(solana_rpc_url);

    let account_pubkey = Pubkey::from_str("9VffBiow5r5YQzgK56rirEWpu45gZGrDWzm9JUt6zL9G").unwrap();
    let acc = client.get_account(&account_pubkey).unwrap();

    print!("acc: {:?}\n\n", acc);

    // &account_pubkey, &acc.owner, &acc.data, None

    let decoded_data = parse_token(&acc.data, None).unwrap();
    print!("decoded_data: {:?}", decoded_data);
    */

    Ok(())
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}
