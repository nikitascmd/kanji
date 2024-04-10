use dotenv::dotenv;
use serde::Deserialize;
use solana_account_decoder::{
    parse_account_data::{self, parse_account_data},
    parse_token::parse_token,
};
use solana_client::{self, rpc_client::RpcClient};
use solana_sdk::{account::Account, entrypoint::deserialize, pubkey::Pubkey};
use std::{env, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let solana_rpc_url = env::var("SOLANA_RPC_URL").unwrap();
    let client = RpcClient::new(solana_rpc_url);

    let account_pubkey = Pubkey::from_str("9VffBiow5r5YQzgK56rirEWpu45gZGrDWzm9JUt6zL9G").unwrap();
    let acc = client.get_account(&account_pubkey).unwrap();

    print!("acc: {:?}\n\n", acc);

    // &account_pubkey, &acc.owner, &acc.data, None

    let decoded_data = parse_token(&acc.data, None).unwrap();
    print!("decoded_data: {:?}", decoded_data);

    Ok(())
}
