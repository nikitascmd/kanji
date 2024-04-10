#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaTokenOverviewResponse {
    pub data: SolanaTokenOverviewData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SolanaTokenOverviewData {
    Data(SolanaTokenOverview),
    Empty(HashMap<String, serde_json::Value>),
}

/// Data field from the /token_overview response from Birdeye
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTokenOverview {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub extensions: Option<Extensions>,
    pub logo_uri: Option<String>,
    pub liquidity: Option<f64>,
    pub price: f64,

    // 30min
    pub history_30m_price: Option<f64>,
    pub price_change_30m_percent: Option<f64>,
    // 1h
    pub history_1h_price: Option<f64>,
    pub price_change_1h_percent: Option<f64>,
    // 2h
    pub history_2h_price: Option<f64>,
    pub price_change_2h_percent: Option<f64>,
    // 4h
    pub history_4h_price: Option<f64>,
    pub price_change_4h_percent: Option<f64>,
    // 6h
    pub history_6h_price: Option<f64>,
    pub price_change_6h_percent: Option<f64>,
    // 8h
    pub history_8h_price: Option<f64>,
    pub price_change_8h_percent: Option<f64>,
    // 12h
    pub history_12h_price: Option<f64>,
    pub price_change_12h_percent: Option<f64>,
    // 24h
    pub history_24h_price: Option<f64>,
    pub price_change_24h_percent: Option<f64>,

    // 30min
    pub unique_wallet_30m: Option<u64>,
    pub unique_wallet_history_30m: Option<u64>,
    pub unique_wallet_30m_change_percent: Option<f64>,
    // 1h
    pub unique_wallet_1h: Option<u64>,
    pub unique_wallet_history_1h: Option<u64>,
    pub unique_wallet_1h_change_percent: Option<f64>,
    // 2h
    pub unique_wallet_2h: Option<u64>,
    pub unique_wallet_history_2h: Option<u64>,
    pub unique_wallet_2h_change_percent: Option<f64>,
    // 4h
    pub unique_wallet_4h: Option<u64>,
    pub unique_wallet_history_4h: Option<u64>,
    pub unique_wallet_4h_change_percent: Option<f64>,
    // 6h
    pub unique_wallet_6h: Option<u64>,
    pub unique_wallet_history_6h: Option<u64>,
    pub unique_wallet_6h_change_percent: Option<f64>,
    // 8h
    pub unique_wallet_8h: Option<u64>,
    pub unique_wallet_history_8h: Option<u64>,
    pub unique_wallet_8h_change_percent: Option<f64>,
    // 12h
    pub unique_wallet_12h: Option<u64>,
    pub unique_wallet_history_12h: Option<u64>,
    pub unique_wallet_12h_change_percent: Option<f64>,
    // 24h
    pub unique_wallet_24h: Option<u64>,
    pub unique_wallet_history_24h: Option<u64>,
    pub unique_wallet_24h_change_percent: Option<f64>,

    pub last_trade_unix_time: Option<i64>,
    pub last_trade_human_time: Option<String>,

    pub supply: Option<f64>,
    pub mc: Option<f64>,

    // 30min
    pub trade_30m: Option<u64>,
    pub trade_history_30m: Option<u64>,
    pub trade_30m_change_percent: Option<f64>,
    pub sell_30m: Option<u64>,
    pub sell_history_30m: Option<u64>,
    pub sell_30m_change_percent: Option<f64>,
    pub buy_30m: Option<u64>,
    pub buy_history_30m: Option<u64>,
    pub buy_30m_change_percent: Option<f64>,
    pub v_30m: Option<f64>,
    pub v_30m_usd: Option<f64>,
    pub v_history_30m: Option<f64>,
    pub v_history_30m_usd: Option<f64>,
    pub v_30m_change_percent: Option<f64>,
    pub v_buy_30m: Option<f64>,
    pub v_buy_30m_usd: Option<f64>,
    pub v_buy_history_30m: Option<f64>,
    pub v_buy_history_30m_usd: Option<f64>,
    pub v_buy_30m_change_percent: Option<f64>,
    pub v_sell_30m: Option<f64>,
    pub v_sell_30m_usd: Option<f64>,
    pub v_sell_history_30m: Option<f64>,
    pub v_sell_history_30m_usd: Option<f64>,
    pub v_sell_30m_change_percent: Option<f64>,
    // 1h
    pub trade_1h: Option<u64>,
    pub trade_history_1h: Option<u64>,
    pub trade_1h_change_percent: Option<f64>,
    pub sell_1h: Option<u64>,
    pub sell_history_1h: Option<u64>,
    pub sell_1h_change_percent: Option<f64>,
    pub buy_1h: Option<u64>,
    pub buy_history_1h: Option<u64>,
    pub buy_1h_change_percent: Option<f64>,
    pub v_1h: Option<f64>,
    pub v_1h_usd: Option<f64>,
    pub v_history_1h: Option<f64>,
    pub v_history_1h_usd: Option<f64>,
    pub v_1h_change_percent: Option<f64>,
    pub v_buy_1h: Option<f64>,
    pub v_buy_1h_usd: Option<f64>,
    pub v_buy_history_1h: Option<f64>,
    pub v_buy_history_1h_usd: Option<f64>,
    pub v_buy_1h_change_percent: Option<f64>,
    pub v_sell_1h: Option<f64>,
    pub v_sell_1h_usd: Option<f64>,
    pub v_sell_history_1h: Option<f64>,
    pub v_sell_history_1h_usd: Option<f64>,
    pub v_sell_1h_change_percent: Option<f64>,
    // 2h
    pub trade_2h: Option<u64>,
    pub trade_history_2h: Option<u64>,
    pub trade_2h_change_percent: Option<f64>,
    pub sell_2h: Option<u64>,
    pub sell_history_2h: Option<u64>,
    pub sell_2h_change_percent: Option<f64>,
    pub buy_2h: Option<u64>,
    pub buy_history_2h: Option<u64>,
    pub buy_2h_change_percent: Option<f64>,
    pub v_2h: Option<f64>,
    pub v_2h_usd: Option<f64>,
    pub v_history_2h: Option<f64>,
    pub v_history_2h_usd: Option<f64>,
    pub v_2h_change_percent: Option<f64>,
    pub v_buy_2h: Option<f64>,
    pub v_buy_2h_usd: Option<f64>,
    pub v_buy_history_2h: Option<f64>,
    pub v_buy_history_2h_usd: Option<f64>,
    pub v_buy_2h_change_percent: Option<f64>,
    pub v_sell_2h: Option<f64>,
    pub v_sell_2h_usd: Option<f64>,
    pub v_sell_history_2h: Option<f64>,
    pub v_sell_history_2h_usd: Option<f64>,
    pub v_sell_2h_change_percent: Option<f64>,

    // 4h
    pub trade_4h: Option<u64>,
    pub trade_history_4h: Option<u64>,
    pub trade_4h_change_percent: Option<f64>,
    pub sell_4h: u64,
    pub sell_history_4h: Option<u64>,
    pub sell_4h_change_percent: Option<f64>,
    pub buy_4h: u64,
    pub buy_history_4h: Option<u64>,
    pub buy_4h_change_percent: Option<f64>,
    pub v_4h: f64,
    pub v_4h_usd: f64,
    pub v_history_4h: Option<f64>,
    pub v_history_4h_usd: Option<f64>,
    pub v_4h_change_percent: Option<f64>,
    pub v_buy_4h: f64,
    pub v_buy_4h_usd: Option<f64>,
    pub v_buy_history_4h: Option<f64>,
    pub v_buy_history_4h_usd: Option<f64>,
    pub v_buy_4h_change_percent: Option<f64>,
    pub v_sell_4h: Option<f64>,
    pub v_sell_4h_usd: Option<f64>,
    pub v_sell_history_4h: Option<f64>,
    pub v_sell_history_4h_usd: Option<f64>,
    pub v_sell_4h_change_percent: Option<f64>,
    // 6h
    pub trade_6h: Option<u64>,
    pub trade_history_6h: Option<u64>,
    pub trade_6h_change_percent: Option<f64>,
    pub sell_6h: Option<u64>,
    pub sell_history_6h: Option<u64>,
    pub sell_6h_change_percent: Option<f64>,
    pub buy_6h: Option<u64>,
    pub buy_history_6h: Option<u64>,
    pub buy_6h_change_percent: Option<f64>,
    pub v_6h: Option<f64>,
    pub v_6h_usd: Option<f64>,
    pub v_history_6h: Option<f64>,
    pub v_history_6h_usd: Option<f64>,
    pub v_6h_change_percent: Option<f64>,
    pub v_buy_6h: Option<f64>,
    pub v_buy_6h_usd: Option<f64>,
    pub v_buy_history_6h: Option<f64>,
    pub v_buy_history_6h_usd: Option<f64>,
    pub v_buy_6h_change_percent: Option<f64>,
    pub v_sell_6h: Option<f64>,
    pub v_sell_6h_usd: Option<f64>,
    pub v_sell_history_6h: Option<f64>,
    pub v_sell_history_6h_usd: Option<f64>,
    pub v_sell_6h_change_percent: Option<f64>,
    // 8h
    pub trade_8h: Option<u64>,
    pub trade_history_8h: Option<u64>,
    pub trade_8h_change_percent: Option<f64>,
    pub sell_8h: Option<u64>,
    pub sell_history_8h: Option<u64>,
    pub sell_8h_change_percent: Option<f64>,
    pub buy_8h: Option<u64>,
    pub buy_history_8h: Option<u64>,
    pub buy_8h_change_percent: Option<f64>,
    pub v_8h: Option<f64>,
    pub v_8h_usd: Option<f64>,
    pub v_history_8h: Option<f64>,
    pub v_history_8h_usd: Option<f64>,
    pub v_8h_change_percent: Option<f64>,
    pub v_buy_8h: Option<f64>,
    pub v_buy_8h_usd: Option<f64>,
    pub v_buy_history_8h: Option<f64>,
    pub v_buy_history_8h_usd: Option<f64>,
    pub v_buy_8h_change_percent: Option<f64>,
    pub v_sell_8h: Option<f64>,
    pub v_sell_8h_usd: Option<f64>,
    pub v_sell_history_8h: Option<f64>,
    pub v_sell_history_8h_usd: Option<f64>,
    pub v_sell_8h_change_percent: Option<f64>,

    // 12h
    pub trade_12h: Option<u64>,
    pub trade_history_12h: Option<u64>,
    pub trade_12h_change_percent: Option<f64>,
    pub sell_12h: Option<u64>,
    pub sell_history_12h: Option<u64>,
    pub sell_12h_change_percent: Option<f64>,
    pub buy_12h: Option<u64>,
    pub buy_history_12h: Option<u64>,
    pub buy_12h_change_percent: Option<f64>,
    pub v_12h: Option<f64>,
    pub v_12h_usd: Option<f64>,
    pub v_history_12h: Option<f64>,
    pub v_history_12h_usd: Option<f64>,
    pub v_12h_change_percent: Option<f64>,
    pub v_buy_12h: Option<f64>,
    pub v_buy_12h_usd: Option<f64>,
    pub v_buy_history_12h: Option<f64>,
    pub v_buy_history_12h_usd: Option<f64>,
    pub v_buy_12h_change_percent: Option<f64>,
    pub v_sell_12h: Option<f64>,
    pub v_sell_12h_usd: Option<f64>,
    pub v_sell_history_12h: Option<f64>,
    pub v_sell_history_12h_usd: Option<f64>,
    pub v_sell_12h_change_percent: Option<f64>,
    // 24h
    pub trade_24h: Option<u64>,
    pub trade_history_24h: Option<u64>,
    pub trade_24h_change_percent: Option<f64>,
    pub sell_24h: Option<u64>,
    pub sell_history_24h: Option<u64>,
    pub sell_24h_change_percent: Option<f64>,
    pub buy_24h: Option<u64>,
    pub buy_history_24h: Option<u64>,
    pub buy_24h_change_percent: Option<f64>,
    pub v_24h: Option<f64>,
    pub v_24h_usd: Option<f64>,
    pub v_history_24h: Option<f64>,
    pub v_history_24h_usd: Option<f64>,
    pub v_24h_change_percent: Option<f64>,
    pub v_buy_24h: Option<f64>,
    pub v_buy_24h_usd: Option<f64>,
    pub v_buy_history_24h: Option<f64>,
    pub v_buy_history_24h_usd: Option<f64>,
    pub v_buy_24h_change_percent: Option<f64>,
    pub v_sell_24h: Option<f64>,
    pub v_sell_24h_usd: Option<f64>,
    pub v_sell_history_24h: Option<f64>,
    pub v_sell_history_24h_usd: Option<f64>,
    pub v_sell_24h_change_percent: Option<f64>,

    pub watch: Option<String>,

    // 30min view
    pub view_30m: Option<u64>,
    pub view_history_30m: Option<u64>,
    pub view_30m_change_percent: Option<f64>,
    // 1h
    pub view_1h: Option<u64>,
    pub view_history_1h: Option<u64>,
    pub view_1h_change_percent: Option<f64>,
    // 2h
    pub view_2h: Option<u64>,
    pub view_history_2h: Option<u64>,
    pub view_2h_change_percent: Option<f64>,
    // 4h
    pub view_4h: Option<u64>,
    pub view_history_4h: Option<u64>,
    pub view_4h_change_percent: Option<f64>,
    // 6h
    pub view_6h: Option<u64>,
    pub view_history_6h: Option<u64>,
    pub view_6h_change_percent: Option<f64>,
    // 8h
    pub view_8h: Option<u64>,
    pub view_history_8h: Option<u64>,
    pub view_8h_change_percent: Option<f64>,
    // 12h
    pub view_12h: Option<u64>,
    pub view_history_12h: Option<u64>,
    pub view_12h_change_percent: Option<f64>,
    // 24h
    pub view_24h: Option<u64>,
    pub view_history_24h: Option<u64>,
    pub view_24h_change_percent: Option<f64>,

    // 30min
    pub unique_view_30m: Option<u64>,
    pub unique_view_history_30m: Option<u64>,
    pub unique_view_30m_change_percent: Option<f64>,
    // 1h
    pub unique_view_1h: Option<u64>,
    pub unique_view_history_1h: Option<u64>,
    pub unique_view_1h_change_percent: Option<f64>,
    // 2h
    pub unique_view_2h: Option<u64>,
    pub unique_view_history_2h: Option<u64>,
    pub unique_view_2h_change_percent: Option<f64>,
    // 4h
    pub unique_view_4h: Option<u64>,
    pub unique_view_history_4h: Option<u64>,
    pub unique_view_4h_change_percent: Option<f64>,
    // 6h
    pub unique_view_6h: Option<u64>,
    pub unique_view_history_6h: Option<u64>,
    pub unique_view_6h_change_percent: Option<f64>,
    // 8h
    pub unique_view_8h: Option<u64>,
    pub unique_view_history_8h: Option<u64>,
    pub unique_view_8h_change_percent: Option<f64>,
    // 12h
    pub unique_view_12h: Option<u64>,
    pub unique_view_history_12h: Option<u64>,
    pub unique_view_12h_change_percent: Option<f64>,
    // 24h
    pub unique_view_24h: Option<u64>,
    pub unique_view_history_24h: Option<u64>,
    pub unique_view_24h_change_percent: Option<f64>,

    pub number_markets: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(flatten)]
    pub properties: HashMap<String, Option<String>>,
}
