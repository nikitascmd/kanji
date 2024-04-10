use serde::{Deserialize, Serialize};

/// Response from: https://api.dexscreener.com/latest/dex/pairs/:chainId/:pairAddresses
#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerPairsResponse {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<DexscreenerPair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pairs: Option<Vec<DexscreenerPair>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DexscreenerPair {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    pub pair_address: String,
    pub base_token: DexscreenerToken,
    pub quote_token: DexscreenerQuoteToken,
    pub price_native: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    pub txns: DexscreenerTransactions,
    pub volume: DexscreenerVolume,
    pub price_change: DexscreenerPriceChange,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<DexscreenerLiquidity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdv: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair_created_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerQuoteToken {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerTransactions {
    pub m5: DexscreenerTransactionDetail,
    pub h1: DexscreenerTransactionDetail,
    pub h6: DexscreenerTransactionDetail,
    pub h24: DexscreenerTransactionDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerTransactionDetail {
    pub buys: i32,
    pub sells: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerVolume {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerPriceChange {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DexscreenerLiquidity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
    pub base: f64,
    pub quote: f64,
}
