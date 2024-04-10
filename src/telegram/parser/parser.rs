use std::env;
use std::str::FromStr;
use super::birdeye_structs::{SolanaTokenOverview, SolanaTokenOverviewResponse};
use super::{DexscreenerPair, DexscreenerPairsResponse, SolanaTokenOverviewData};
use crate::telegram::{TelegramAccount, TelegramGroup};
use grammers_client::types::Message;
use regex::Regex;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};

#[derive(Debug)]
pub struct ParseResult {
    token: Token,
    call: Call,
    telegram_user: TelegramUser,
}

#[derive(Debug)]
pub struct Token {
    address: String,
    decimals: u8,
    name: String,
    symbol: String,
    network: String,
}

#[derive(Debug)]
pub struct Call {
    timestamp: String,
    is_channel_call: bool,
    message_text: String,
    birdeye_data: SolanaTokenOverview,
}

#[derive(Debug)]
pub struct TelegramUser {
    telegram_id: u64, // as per gramme.rs library
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
}

pub struct DefaultParser {
    telegram_account: TelegramAccount,
    solana_regex: Regex,
    birdeye_api_key: String,
    reqwest_client: reqwest::Client,
    solana_client: RpcClient
}

enum SolanaAccountType {
    Token,
    RaydiumPair,
    Other 
}

impl DefaultParser {
    pub fn new(telegram_account:  TelegramAccount) -> Self {
        let solana_regex = Regex::new(r"[1-9A-HJ-NP-Za-km-z]{32,44}").unwrap();
        let birdeye_api_key = env::var("BIRDEYE_API_KEY").unwrap();
        let reqwest_client = reqwest::Client::new();
        let solana_client = RpcClient::new(env::var("SOLANA_RPC_URL").unwrap());

        Self {
            telegram_account,
            solana_regex,
            birdeye_api_key,
            reqwest_client,
            solana_client
        }
    }

    pub async fn parse(
        &self,
        message: Message,
    ) -> Result<Option<ParseResult>, Box<dyn std::error::Error>> {
        // 1. Ignore messages in chats we are not tracking
        if !self.telegram_account.tracked_groups.contains_key(&message.chat().id()) {
            return Ok(None);
        }

        // 2. Ignore messages from Telegram bots
        if message.via_bot_id().is_some() {
            return Ok(None);
        }

        // 3. Extract token from message. If no token ignore
        let token = self.extract_token_data(message).await?;

        // 4. Extract Telegram User that made the call

        todo!()
        // Ok(Some())
    }

    async fn extract_token_data(
        &self,
        message: Message,
    ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
        // 1. Try extract token from message text by searching for solana addresses
        let solana_addresses: Vec<&str> = self
            .solana_regex
            .find_iter(message.text())
            .map(|mat| mat.as_str())
            .collect();

        // For each found solana address try to query /token_overview
        // /token_overview requires a *token_address* and not a *pair_address*,
        // so, if first /token_overview fetch fails, we query with `fetch_token_overview_from_pair`
        if !solana_addresses.is_empty() {
            for address in solana_addresses {
                match self.fetch_token_overview(address).await {
                    Ok(Some(data)) => return Ok(Some(data)),
                    _ => {
                        match self.fetch_token_overview_from_pair(address).await {
                            Ok(Some(data)) => return Ok(Some(data)),
                            _ => continue,
                        };
                    }
                };
            }
        }

        // 2. Try extracting token from entities like hyperlinks
        Ok(None)
    }

    async fn try_get_token_overview(&self, address:&str) -> Option<SolanaTokenOverview> {
        // 1. Determine type of solana account (address)
        let address_type = 
    }

    async fn determine_account_type(&self, address: &str) -> SolanaAccountType {
        // 1. Form the solana public key for the inputted address
        let pub_key = match Pubkey::from_str(address) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error while trying to form public key for solana address: {} \nError: {}", address, e);
                return SolanaAccountType::Other
            }
        };

        // 2. Fetch solana account info associated with inputted address
        let acc = match self.solana_client.get_account(&pub_key) {
            Ok(acc) => acc,
            Err(e) => {
                eprintln!("Error while trying to get_account for solana address: {} \nError: {}", address, e);
                return SolanaAccountType::Other}
        };
        
        // 3. Attempt to fetch token info. If it's not a token it will throw error

        // Raydium pair always has the same owner of Raydium Liq Pool V4: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
              
        




        Ok()
    }

    async fn fetch_token_overview(
        &self,
        token_address: &str,
    ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
        let birdeye_token_overview_query = format!(
            "https://public-api.birdeye.so/defi/token_overview?address={}",
            token_address
        );

        let token_overview_response: SolanaTokenOverviewResponse = self
            .reqwest_client
            .get(&birdeye_token_overview_query)
            .header("x-chain", "solana")
            .header("X-API-KEY", &self.birdeye_api_key)
            .send()
            .await?
            .json()
            .await?;

        if !token_overview_response.success {
            return Ok(None);
        }

        match token_overview_response.data {
            SolanaTokenOverviewData::Data(data) => Ok(Some(data)),
            SolanaTokenOverviewData::Empty(_) => Ok(None),
        }
    }

    async fn fetch_token_overview_from_pair(
        &self,
        pair_address: &str,
    ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
        // Find the address of the token by using the pair address
        // because GET /token_overview requires token address, NOT pair address
        match self.fetch_pair(pair_address).await? {
            Some(pair) => Ok(self.fetch_token_overview(&pair.base_token.address).await?),
            None => Ok(None),
        }
    }

    async fn fetch_pair(
        &self,
        pair_address: &str,
    ) -> Result<Option<DexscreenerPair>, Box<dyn std::error::Error>> {
        let dexscreener_pairs_query = format!(
            "https://api.dexscreener.com/latest/dex/pairs/solana/{}",
            pair_address
        );

        let dexscreener_pair_response: DexscreenerPairsResponse = self
            .reqwest_client
            .get(&dexscreener_pairs_query)
            .send()
            .await?
            .json()
            .await?;

        if dexscreener_pair_response.pair.is_none() {
            return Ok(None);
        }

        Ok(dexscreener_pair_response.pair)
    }

    async fn (&self, token_address: &str) -> bool {

    }
}
