use super::birdeye_structs::{SolanaTokenOverview, SolanaTokenOverviewResponse};
use super::{DexscreenerPairsResponse, Extensions, SolanaTokenCall, SolanaTokenOverviewData};
use crate::telegram::TelegramAccount;
use grammers_client::types::{Chat, Message};
use regex::Regex;
use solana_account_decoder::parse_token::parse_token;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseResult<'a> {
    pub token: Token,
    pub call: Call,
    pub sender: TelegramSender<'a>,
}

#[derive(Debug)]
pub struct Token {
    pub address: String,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub network: String,
    pub logo_uri: Option<String>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug)]
pub struct Call {
    pub timestamp: String,
    pub is_channel_call: bool,
    pub message_text: String,
    pub token_call_data: SolanaTokenCall,
}

#[derive(Debug)]
pub struct TelegramSender<'a> {
    pub telegram_id: i64, // as per gramme.rs library
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub username: Option<&'a str>, // for channels this is the title
    pub is_channel: bool,
}

pub struct DefaultParser {
    telegram_account: TelegramAccount,
    solana_regex: Regex,
    birdeye_api_key: String,
    reqwest_client: reqwest::Client,
    solana_client: RpcClient,
}

enum SolanaAccountType {
    Token,
    RaydiumPair,
    Other,
}

impl DefaultParser {
    pub fn new(telegram_account: TelegramAccount) -> Self {
        let solana_regex = Regex::new(r"[1-9A-HJ-NP-Za-km-z]{32,44}").unwrap();
        let birdeye_api_key = env::var("BIRDEYE_API_KEY").unwrap();
        let reqwest_client = reqwest::Client::new();
        let solana_client = RpcClient::new(env::var("SOLANA_RPC_URL").unwrap());

        Self {
            telegram_account,
            solana_regex,
            birdeye_api_key,
            reqwest_client,
            solana_client,
        }
    }

    pub async fn parse(
        &self,
        message: Message,
    ) -> Result<Option<ParseResult>, Box<dyn std::error::Error>> {
        // 1. Ignore messages in chats we are not tracking
        if !self
            .telegram_account
            .tracked_groups
            .contains_key(&message.chat().id())
        {
            return Ok(None);
        }

        // 2. Ignore messages from Telegram bots
        if message.via_bot_id().is_some() {
            return Ok(None);
        }

        // 3. Extract token data from message. If no token exit function
        let (token, call) = match self.extract_token_data(&message).await {
            Ok(Some(token)) => self.format_token_data(token),
            Err(err) => {
                eprint!("Error while parsing: {}", err);
                return Ok(None);
            }
            _ => {
                println!("Could not find token in message");
                return Ok(None);
            }
        };

        // 4. Extract Telegram User that made the call. If no sender exit function
        let sender = match self.extract_sender(&message).await {
            Some(sender) => sender,
            _ => {
                eprintln!("Could not fetch the Sender for the message...");
                return Ok(None);
            }
        };

        // 5. From Call
        let call = Call {
            timestamp: message.date().to_string(),
            is_channel_call: sender.is_channel,
            token_call_data: call,
            message_text: message.text().to_string(),
        };

        let parse_result = ParseResult {
            token,
            call,
            sender,
        };

        Ok(Some(parse_result))
    }

    async fn extract_sender(&self, message: &Message) -> Option<TelegramSender> {
        match message.sender() {
            Some(Chat::User(user)) => TelegramSender {
                telegram_id: user.id(),
                first_name: Some(user.first_name()),
                last_name: user.last_name(),
                username: user.username(),
                is_channel: false,
            },
            Some(Chat::Channel(channel)) => TelegramSender {
                telegram_id: channel.id(),
                username: Some(channel.title()),
                first_name: None,
                last_name: None,
                is_channel: true,
            },
            Some(Chat::Group(group)) => {
                println!("message.sender() returned group: {:?}", group);
                return None;
            }
            None => return None,
        };

        None
    }

    async fn extract_token_data(
        &self,
        message: &Message,
    ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
        // 1. Try extract token from message text by searching for solana addresses
        let solana_addresses: Vec<&str> = self
            .solana_regex
            .find_iter(message.text())
            .map(|mat| mat.as_str())
            .collect();

        // 2. For each found solana address try to query birdeye /token_overview
        if !solana_addresses.is_empty() {
            if let Some(data) = self.try_get_token_overview(&solana_addresses).await {
                return Ok(Some(data));
            }
        }

        // 3. Try extracting token from entities like hyperlinks
        Ok(None)
    }

    async fn try_get_token_overview(&self, addresses: &[&str]) -> Option<SolanaTokenOverview> {
        for address in addresses {
            // 1. Determine type of solana account for that address. We are only interested in tokens/pairs
            // the GET /token_overview from birdeye accepts only token addresses and not pairs,
            // so if it's a pair we need to get the token address from it
            let token_overview = match self.determine_account_type(address).await {
                SolanaAccountType::Token => self.fetch_token_overview(address).await,
                SolanaAccountType::RaydiumPair => {
                    self.fetch_token_overview_from_pair(address).await
                }
                SolanaAccountType::Other => return None,
            };

            // 2.  Upon successful fetch, assume the first one is the correct token and return it instantly
            if let Ok(Some(data)) = token_overview {
                return Some(data);
            }
        }

        // 3. If none of the fetches in the loop were successful, means none of the addresses were tokens/pairs
        return None;
    }

    async fn determine_account_type(&self, address: &str) -> SolanaAccountType {
        // 1. Form the solana public key for the inputted address
        let pub_key = match Pubkey::from_str(address) {
            Ok(value) => value,
            Err(e) => {
                eprintln!(
                    "Error while trying to form public key for solana address: {} \nError: {}",
                    address, e
                );
                return SolanaAccountType::Other;
            }
        };

        // 2. Fetch solana account info associated with inputted address
        let account = match self.solana_client.get_account(&pub_key) {
            Ok(account) => account,
            Err(e) => {
                eprintln!(
                    "Error while trying to get_account for solana address: {} \nError: {}",
                    address, e
                );
                return SolanaAccountType::Other;
            }
        };

        // 3. Raydium pairs always have the same owner of Raydium Liq Pool V4: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
        const RAYDIUM_LIQUIDITY_POOL_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
        if account.owner.to_string() == RAYDIUM_LIQUIDITY_POOL_V4 {
            return SolanaAccountType::RaydiumPair;
        }

        // 4. Attempt to fetch token info. If it's not a token it will throw error
        match parse_token(&account.data, None) {
            Ok(_) => return SolanaAccountType::Token,
            _ => return SolanaAccountType::Other,
        };
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
        // Construct the URL to fetch the pair details
        let dexscreener_pairs_query = format!(
            "https://api.dexscreener.com/latest/dex/pairs/solana/{}",
            pair_address
        );

        // Make the HTTP request to get the pair
        let dexscreener_pair_response: DexscreenerPairsResponse = self
            .reqwest_client
            .get(&dexscreener_pairs_query)
            .send()
            .await?
            .json()
            .await?;

        // Process the response to possibly get the base token's overview
        match dexscreener_pair_response.pair {
            Some(pair) => {
                // If pair exists, fetch the token overview using the base token address
                Ok(self.fetch_token_overview(&pair.base_token.address).await?)
            }
            None => {
                // If no pair is found, return None
                Ok(None)
            }
        }
    }

    pub fn format_token_data(&self, data: SolanaTokenOverview) -> (Token, SolanaTokenCall) {
        let token = Token {
            address: data.address.clone(),
            decimals: data.decimals,
            name: data.name.clone(),
            symbol: data.symbol.clone(),
            network: String::from("solana"), // Assuming Solana network
            logo_uri: data.logo_uri.clone(),
            extensions: data.extensions,
        };

        let solana_call_data = SolanaTokenCall {
            liquidity: data.liquidity,
            price: data.price,
            history_30m_price: data.history_30m_price,
            price_change_30m_percent: data.price_change_30m_percent,
            history_1h_price: data.history_1h_price,
            price_change_1h_percent: data.price_change_1h_percent,
            history_2h_price: data.history_2h_price,
            price_change_2h_percent: data.price_change_2h_percent,
            history_4h_price: data.history_4h_price,
            price_change_4h_percent: data.price_change_4h_percent,
            history_6h_price: data.history_6h_price,
            price_change_6h_percent: data.price_change_6h_percent,
            history_8h_price: data.history_8h_price,
            price_change_8h_percent: data.price_change_8h_percent,
            history_12h_price: data.history_12h_price,
            price_change_12h_percent: data.price_change_12h_percent,
            history_24h_price: data.history_24h_price,
            price_change_24h_percent: data.price_change_24h_percent,
            unique_wallet_30m: data.unique_wallet_30m,
            unique_wallet_history_30m: data.unique_wallet_history_30m,
            unique_wallet_30m_change_percent: data.unique_wallet_30m_change_percent,
            unique_wallet_1h: data.unique_wallet_1h,
            unique_wallet_history_1h: data.unique_wallet_history_1h,
            unique_wallet_1h_change_percent: data.unique_wallet_1h_change_percent,
            unique_wallet_2h: data.unique_wallet_2h,
            unique_wallet_history_2h: data.unique_wallet_history_2h,
            unique_wallet_2h_change_percent: data.unique_wallet_2h_change_percent,
            unique_wallet_4h: data.unique_wallet_4h,
            unique_wallet_history_4h: data.unique_wallet_history_4h,
            unique_wallet_4h_change_percent: data.unique_wallet_4h_change_percent,
            unique_wallet_6h: data.unique_wallet_6h,
            unique_wallet_history_6h: data.unique_wallet_history_6h,
            unique_wallet_6h_change_percent: data.unique_wallet_6h_change_percent,
            unique_wallet_8h: data.unique_wallet_8h,
            unique_wallet_history_8h: data.unique_wallet_history_8h,
            unique_wallet_8h_change_percent: data.unique_wallet_8h_change_percent,
            unique_wallet_12h: data.unique_wallet_12h,
            unique_wallet_history_12h: data.unique_wallet_history_12h,
            unique_wallet_12h_change_percent: data.unique_wallet_12h_change_percent,
            unique_wallet_24h: data.unique_wallet_24h,
            unique_wallet_history_24h: data.unique_wallet_history_24h,
            unique_wallet_24h_change_percent: data.unique_wallet_24h_change_percent,
            last_trade_unix_time: data.last_trade_unix_time,
            last_trade_human_time: data.last_trade_human_time.clone(),
            supply: data.supply,
            mc: data.mc,
            trade_30m: data.trade_30m,
            trade_history_30m: data.trade_history_30m,
            trade_30m_change_percent: data.trade_30m_change_percent,
            sell_30m: data.sell_30m,
            sell_history_30m: data.sell_history_30m,
            sell_30m_change_percent: data.sell_30m_change_percent,
            buy_30m: data.buy_30m,
            buy_history_30m: data.buy_history_30m,
            buy_30m_change_percent: data.buy_30m_change_percent,
            v_30m: data.v_30m,
            v_30m_usd: data.v_30m_usd,
            v_history_30m: data.v_history_30m,
            v_history_30m_usd: data.v_history_30m_usd,
            v_30m_change_percent: data.v_30m_change_percent,
            v_buy_30m: data.v_buy_30m,
            v_buy_30m_usd: data.v_buy_30m_usd,
            v_buy_history_30m: data.v_buy_history_30m,
            v_buy_history_30m_usd: data.v_buy_history_30m_usd,
            v_buy_30m_change_percent: data.v_buy_30m_change_percent,
            v_sell_30m: data.v_sell_30m,
            v_sell_30m_usd: data.v_sell_30m_usd,
            v_sell_history_30m: data.v_sell_history_30m,
            v_sell_history_30m_usd: data.v_sell_history_30m_usd,
            v_sell_30m_change_percent: data.v_sell_30m_change_percent,
            trade_1h: data.trade_1h,
            trade_history_1h: data.trade_history_1h,
            trade_1h_change_percent: data.trade_1h_change_percent,
            sell_1h: data.sell_1h,
            sell_history_1h: data.sell_history_1h,
            sell_1h_change_percent: data.sell_1h_change_percent,
            buy_1h: data.buy_1h,
            buy_history_1h: data.buy_history_1h,
            buy_1h_change_percent: data.buy_1h_change_percent,
            v_1h: data.v_1h,
            v_1h_usd: data.v_1h_usd,
            v_history_1h: data.v_history_1h,
            v_history_1h_usd: data.v_history_1h_usd,
            v_1h_change_percent: data.v_1h_change_percent,
            v_buy_1h: data.v_buy_1h,
            v_buy_1h_usd: data.v_buy_1h_usd,
            v_buy_history_1h: data.v_buy_history_1h,
            v_buy_history_1h_usd: data.v_buy_history_1h_usd,
            v_buy_1h_change_percent: data.v_buy_1h_change_percent,
            v_sell_1h: data.v_sell_1h,
            v_sell_1h_usd: data.v_sell_1h_usd,
            v_sell_history_1h: data.v_sell_history_1h,
            v_sell_history_1h_usd: data.v_sell_history_1h_usd,
            v_sell_1h_change_percent: data.v_sell_1h_change_percent,
            trade_2h: data.trade_2h,
            trade_history_2h: data.trade_history_2h,
            trade_2h_change_percent: data.trade_2h_change_percent,
            sell_2h: data.sell_2h,
            sell_history_2h: data.sell_history_2h,
            sell_2h_change_percent: data.sell_2h_change_percent,
            buy_2h: data.buy_2h,
            buy_history_2h: data.buy_history_2h,
            buy_2h_change_percent: data.buy_2h_change_percent,
            v_2h: data.v_2h,
            v_2h_usd: data.v_2h_usd,
            v_history_2h: data.v_history_2h,
            v_history_2h_usd: data.v_history_2h_usd,
            v_2h_change_percent: data.v_2h_change_percent,
            v_buy_2h: data.v_buy_2h,
            v_buy_2h_usd: data.v_buy_2h_usd,
            v_buy_history_2h: data.v_buy_history_2h,
            v_buy_history_2h_usd: data.v_buy_history_2h_usd,
            v_buy_2h_change_percent: data.v_buy_2h_change_percent,
            v_sell_2h: data.v_sell_2h,
            v_sell_2h_usd: data.v_sell_2h_usd,
            v_sell_history_2h: data.v_sell_history_2h,
            v_sell_history_2h_usd: data.v_sell_history_2h_usd,
            v_sell_2h_change_percent: data.v_sell_2h_change_percent,
            trade_4h: data.trade_4h,
            trade_history_4h: data.trade_history_4h,
            trade_4h_change_percent: data.trade_4h_change_percent,
            sell_4h: data.sell_4h,
            sell_history_4h: data.sell_history_4h,
            sell_4h_change_percent: data.sell_4h_change_percent,
            buy_4h: data.buy_4h,
            buy_history_4h: data.buy_history_4h,
            buy_4h_change_percent: data.buy_4h_change_percent,
            v_4h: data.v_4h,
            v_4h_usd: data.v_4h_usd,
            v_history_4h: data.v_history_4h,
            v_history_4h_usd: data.v_history_4h_usd,
            v_4h_change_percent: data.v_4h_change_percent,
            v_buy_4h: data.v_buy_4h,
            v_buy_4h_usd: data.v_buy_4h_usd,
            v_buy_history_4h: data.v_buy_history_4h,
            v_buy_history_4h_usd: data.v_buy_history_4h_usd,
            v_buy_4h_change_percent: data.v_buy_4h_change_percent,
            v_sell_4h: data.v_sell_4h,
            v_sell_4h_usd: data.v_sell_4h_usd,
            v_sell_history_4h: data.v_sell_history_4h,
            v_sell_history_4h_usd: data.v_sell_history_4h_usd,
            v_sell_4h_change_percent: data.v_sell_4h_change_percent,
            trade_6h: data.trade_6h,
            trade_history_6h: data.trade_history_6h,
            trade_6h_change_percent: data.trade_6h_change_percent,
            sell_6h: data.sell_6h,
            sell_history_6h: data.sell_history_6h,
            sell_6h_change_percent: data.sell_6h_change_percent,
            buy_6h: data.buy_6h,
            buy_history_6h: data.buy_history_6h,
            buy_6h_change_percent: data.buy_6h_change_percent,
            v_6h: data.v_6h,
            v_6h_usd: data.v_6h_usd,
            v_history_6h: data.v_history_6h,
            v_history_6h_usd: data.v_history_6h_usd,
            v_6h_change_percent: data.v_6h_change_percent,
            v_buy_6h: data.v_buy_6h,
            v_buy_6h_usd: data.v_buy_6h_usd,
            v_buy_history_6h: data.v_buy_history_6h,
            v_buy_history_6h_usd: data.v_buy_history_6h_usd,
            v_buy_6h_change_percent: data.v_buy_6h_change_percent,
            v_sell_6h: data.v_sell_6h,
            v_sell_6h_usd: data.v_sell_6h_usd,
            v_sell_history_6h: data.v_sell_history_6h,
            v_sell_history_6h_usd: data.v_sell_history_6h_usd,
            v_sell_6h_change_percent: data.v_sell_6h_change_percent,
            trade_8h: data.trade_8h,
            trade_history_8h: data.trade_history_8h,
            trade_8h_change_percent: data.trade_8h_change_percent,
            sell_8h: data.sell_8h,
            sell_history_8h: data.sell_history_8h,
            sell_8h_change_percent: data.sell_8h_change_percent,
            buy_8h: data.buy_8h,
            buy_history_8h: data.buy_history_8h,
            buy_8h_change_percent: data.buy_8h_change_percent,
            v_8h: data.v_8h,
            v_8h_usd: data.v_8h_usd,
            v_history_8h: data.v_history_8h,
            v_history_8h_usd: data.v_history_8h_usd,
            v_8h_change_percent: data.v_8h_change_percent,
            v_buy_8h: data.v_buy_8h,
            v_buy_8h_usd: data.v_buy_8h_usd,
            v_buy_history_8h: data.v_buy_history_8h,
            v_buy_history_8h_usd: data.v_buy_history_8h_usd,
            v_buy_8h_change_percent: data.v_buy_8h_change_percent,
            v_sell_8h: data.v_sell_8h,
            v_sell_8h_usd: data.v_sell_8h_usd,
            v_sell_history_8h: data.v_sell_history_8h,
            v_sell_history_8h_usd: data.v_sell_history_8h_usd,
            v_sell_8h_change_percent: data.v_sell_8h_change_percent,
            trade_12h: data.trade_12h,
            trade_history_12h: data.trade_history_12h,
            trade_12h_change_percent: data.trade_12h_change_percent,
            sell_12h: data.sell_12h,
            sell_history_12h: data.sell_history_12h,
            sell_12h_change_percent: data.sell_12h_change_percent,
            buy_12h: data.buy_12h,
            buy_history_12h: data.buy_history_12h,
            buy_12h_change_percent: data.buy_12h_change_percent,
            v_12h: data.v_12h,
            v_12h_usd: data.v_12h_usd,
            v_history_12h: data.v_history_12h,
            v_history_12h_usd: data.v_history_12h_usd,
            v_12h_change_percent: data.v_12h_change_percent,
            v_buy_12h: data.v_buy_12h,
            v_buy_12h_usd: data.v_buy_12h_usd,
            v_buy_history_12h: data.v_buy_history_12h,
            v_buy_history_12h_usd: data.v_buy_history_12h_usd,
            v_buy_12h_change_percent: data.v_buy_12h_change_percent,
            v_sell_12h: data.v_sell_12h,
            v_sell_12h_usd: data.v_sell_12h_usd,
            v_sell_history_12h: data.v_sell_history_12h,
            v_sell_history_12h_usd: data.v_sell_history_12h_usd,
            v_sell_12h_change_percent: data.v_sell_12h_change_percent,
            trade_24h: data.trade_24h,
            trade_history_24h: data.trade_history_24h,
            trade_24h_change_percent: data.trade_24h_change_percent,
            sell_24h: data.sell_24h,
            sell_history_24h: data.sell_history_24h,
            sell_24h_change_percent: data.sell_24h_change_percent,
            buy_24h: data.buy_24h,
            buy_history_24h: data.buy_history_24h,
            buy_24h_change_percent: data.buy_24h_change_percent,
            v_24h: data.v_24h,
            v_24h_usd: data.v_24h_usd,
            v_history_24h: data.v_history_24h,
            v_history_24h_usd: data.v_history_24h_usd,
            v_24h_change_percent: data.v_24h_change_percent,
            v_buy_24h: data.v_buy_24h,
            v_buy_24h_usd: data.v_buy_24h_usd,
            v_buy_history_24h: data.v_buy_history_24h,
            v_buy_history_24h_usd: data.v_buy_history_24h_usd,
            v_buy_24h_change_percent: data.v_buy_24h_change_percent,
            v_sell_24h: data.v_sell_24h,
            v_sell_24h_usd: data.v_sell_24h_usd,
            v_sell_history_24h: data.v_sell_history_24h,
            v_sell_history_24h_usd: data.v_sell_history_24h_usd,
            v_sell_24h_change_percent: data.v_sell_24h_change_percent,
            number_markets: data.number_markets,
            watch: data.watch,
            view_30m: data.view_30m,
            view_history_30m: data.view_history_30m,
            view_30m_change_percent: data.view_30m_change_percent,
            view_1h: data.view_1h,
            view_history_1h: data.view_history_1h,
            view_1h_change_percent: data.view_1h_change_percent,
            view_2h: data.view_2h,
            view_history_2h: data.view_history_2h,
            view_2h_change_percent: data.view_2h_change_percent,
            view_4h: data.view_4h,
            view_history_4h: data.view_history_4h,
            view_4h_change_percent: data.view_4h_change_percent,
            view_6h: data.view_6h,
            view_history_6h: data.view_history_6h,
            view_6h_change_percent: data.view_6h_change_percent,
            view_8h: data.view_8h,
            view_history_8h: data.view_history_8h,
            view_8h_change_percent: data.view_8h_change_percent,
            view_12h: data.view_12h,
            view_history_12h: data.view_history_12h,
            view_12h_change_percent: data.view_12h_change_percent,
            view_24h: data.view_24h,
            view_history_24h: data.view_history_24h,
            view_24h_change_percent: data.view_24h_change_percent,
            unique_view_30m: data.unique_view_30m,
            unique_view_history_30m: data.unique_view_history_30m,
            unique_view_30m_change_percent: data.unique_view_30m_change_percent,
            unique_view_1h: data.unique_view_1h,
            unique_view_history_1h: data.unique_view_history_1h,
            unique_view_1h_change_percent: data.unique_view_1h_change_percent,
            unique_view_2h: data.unique_view_2h,
            unique_view_history_2h: data.unique_view_history_2h,
            unique_view_2h_change_percent: data.unique_view_2h_change_percent,
            unique_view_4h: data.unique_view_4h,
            unique_view_history_4h: data.unique_view_history_4h,
            unique_view_4h_change_percent: data.unique_view_4h_change_percent,
            unique_view_6h: data.unique_view_6h,
            unique_view_history_6h: data.unique_view_history_6h,
            unique_view_6h_change_percent: data.unique_view_6h_change_percent,
            unique_view_8h: data.unique_view_8h,
            unique_view_history_8h: data.unique_view_history_8h,
            unique_view_8h_change_percent: data.unique_view_8h_change_percent,
            unique_view_12h: data.unique_view_12h,
            unique_view_history_12h: data.unique_view_history_12h,
            unique_view_12h_change_percent: data.unique_view_12h_change_percent,
            unique_view_24h: data.unique_view_24h,
            unique_view_history_24h: data.unique_view_history_24h,
            unique_view_24h_change_percent: data.unique_view_24h_change_percent,
        };

        (token, solana_call_data)
    }
}
