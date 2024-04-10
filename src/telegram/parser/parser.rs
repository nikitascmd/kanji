// use super::birdeye_structs::{SolanaTokenOverview, SolanaTokenOverviewResponse};
// use super::{DexscreenerPair, DexscreenerPairsResponse, SolanaTokenOverviewData};
// use crate::telegram::TelegramGroup;
// use grammers_client::types::Message;
// use regex::Regex;
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, env};

// #[derive(Debug)]
// pub struct ParseResult {
//     token: Token,
//     call: Call,
//     telegram_user: TelegramUser,
// }

// #[derive(Debug)]
// pub struct Token {
//     address: String,
//     decimals: u8,
//     name: String,
//     symbol: String,
//     network: String,
// }

// #[derive(Debug)]
// pub struct Call {
//     timestamp: String,
//     is_channel_call: bool,
//     message_text: String,
//     birdeye_data: SolanaTokenOverview,
// }

// #[derive(Debug)]
// pub struct TelegramUser {
//     telegram_id: u64,
//     first_name: Option<String>,
//     last_name: Option<String>,
//     username: Option<String>,
// }

// pub struct DefaultParser {
//     tracked_groups: HashMap<i64, TelegramGroup>,
//     solana_regex: Regex,
//     birdeye_api_key: String,
//     reqwest_client: reqwest::Client,
// }

// impl DefaultParser {
//     pub fn new(tracked_groups: Vec<TelegramGroup>) -> Self {

//         let tracked_groups_map = tracked_groups
//             .into_iter()
//             .map(|group| (group.telegram_id, group))
//             .collect();

//         let solana_regex = Regex::new(r"[1-9A-HJ-NP-Za-km-z]{32,44}").unwrap();
//         let birdeye_api_key = env::var("BIRDEYE_API_KEY").unwrap();
//         let reqwest_client = reqwest::Client::new();

//         DefaultParser {
//             tracked_groups: tracked_groups_map,
//             solana_regex,
//             birdeye_api_key,
//             reqwest_client,
//         }
//     }

//     pub async fn parse(
//         &self,
//         message: Message,
//     ) -> Result<Option<ParseResult>, Box<dyn std::error::Error>> {
//         // 1. Ignore messages in chats we are not tracking
//         if !self.tracked_groups.contains_key(&message.chat().id()) {
//             return Ok(None);
//         }

//         // 2. Ignore messages from Telegram bots
//         if message.via_bot_id().is_some() {
//             return Ok(None);
//         }

//         // 3. Extract token from message. If no token ignore
//         let token = self.extract_token_data(message).await?;

//         // 4. Extract Telegram User that made the call

//         todo!()
//         // Ok(Some())
//     }

//     async fn extract_token_data(
//         &self,
//         message: Message,
//     ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
//         // 1. Try extract token from message text by searching for solana addresses
//         let solana_addresses: Vec<&str> = self
//             .solana_regex
//             .find_iter(message.text())
//             .map(|mat| mat.as_str())
//             .collect();

//         // For each found solana address try to query /token_overview
//         // /token_overview requires a *token_address* and not a *pair_address*,
//         // so, if first /token_overview fetch fails,
//         if !solana_addresses.is_empty() {
//             for address in solana_addresses {
//                 match self.fetch_token_overview(address).await {
//                     Ok(Some(data)) => return Ok(Some(data)),
//                     _ => {
//                         match self.fetch_token_overview_from_pair(address).await {
//                             Ok(Some(data)) => return Ok(Some(data)),
//                             _ => continue,
//                         };
//                     }
//                 };
//             }
//         }

//         // 2. Try extracting token from entities like hyperlinks
//         Ok(None)
//     }

//     async fn fetch_token_overview(
//         &self,
//         token_address: &str,
//     ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
//         let birdeye_token_overview_query = format!(
//             "https://public-api.birdeye.so/defi/token_overview?address={}",
//             token_address
//         );

//         let token_overview_response: SolanaTokenOverviewResponse = self
//             .reqwest_client
//             .get(&birdeye_token_overview_query)
//             .header("x-chain", "solana")
//             .header("X-API-KEY", &self.birdeye_api_key)
//             .send()
//             .await?
//             .json()
//             .await?;

//         if !token_overview_response.success {
//             return Ok(None);
//         }

//         match token_overview_response.data {
//             SolanaTokenOverviewData::Data(data) => Ok(Some(data)),
//             SolanaTokenOverviewData::Empty(_) => Ok(None),
//         }
//     }

//     async fn fetch_token_overview_from_pair(
//         &self,
//         pair_address: &str,
//     ) -> Result<Option<SolanaTokenOverview>, Box<dyn std::error::Error>> {
//         // Find the address of the token by using the pair address
//         // because GET /token_overview requires token address, NOT pair address
//         match self.fetch_pair(pair_address).await? {
//             Some(pair) => Ok(self.fetch_token_overview(&pair.base_token.address).await?),
//             None => Ok(None),
//         }
//     }

//     async fn fetch_pair(
//         &self,
//         pair_address: &str,
//     ) -> Result<Option<DexscreenerPair>, Box<dyn std::error::Error>> {
//         let dexscreener_pairs_query = format!(
//             "https://api.dexscreener.com/latest/dex/pairs/solana/{}",
//             pair_address
//         );

//         let dexscreener_pair_response: DexscreenerPairsResponse = self
//             .reqwest_client
//             .get(&dexscreener_pairs_query)
//             .send()
//             .await?
//             .json()
//             .await?;

//         if dexscreener_pair_response.pair.is_none() {
//             return Ok(None);
//         }

//         Ok(dexscreener_pair_response.pair)
//     }

//     async fn (&self, token_address: &str) -> bool {

//     }
// }
