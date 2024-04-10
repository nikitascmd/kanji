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
/// We use camel case naming intentionally for the serde to deserialize properly,
/// since otherwise we would need to add macro for rename for each of all these fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaTokenOverview {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub extensions: Option<Extensions>,
    pub logoUri: Option<String>,
    pub liquidity: Option<f64>,
    pub price: f64,

    // 30min
    pub history30mPrice: Option<f64>,
    pub priceChange30mPercent: Option<f64>,
    // 1h
    pub history1hPrice: Option<f64>,
    pub priceChange1hPercent: Option<f64>,
    // 2h
    pub history2hPrice: Option<f64>,
    pub priceChange2hPercent: Option<f64>,
    // 4h
    pub history4hPrice: Option<f64>,
    pub priceChange4hPercent: Option<f64>,
    // 6h
    pub history6hPrice: Option<f64>,
    pub priceChange6hPercent: Option<f64>,
    // 8h
    pub history8hPrice: Option<f64>,
    pub priceChange8hPercent: Option<f64>,
    // 12h
    pub history12hPrice: Option<f64>,
    pub priceChange12hPercent: Option<f64>,
    // 24h
    pub history24hPrice: Option<f64>,
    pub priceChange24hPercent: Option<f64>,

    // 30min
    pub uniqueWallet30m: Option<u64>,
    pub uniqueWalletHistory30m: Option<u64>,
    pub uniqueWallet30mChangePercent: Option<f64>,
    // 1h
    pub uniqueWallet1h: Option<u64>,
    pub uniqueWalletHistory1h: Option<u64>,
    pub uniqueWallet1hChangePercent: Option<f64>,
    // 2h
    pub uniqueWallet2h: Option<u64>,
    pub uniqueWalletHistory2h: Option<u64>,
    pub uniqueWallet2hChangePercent: Option<f64>,
    // 4h
    pub uniqueWallet4h: Option<u64>,
    pub uniqueWalletHistory4h: Option<u64>,
    pub uniqueWallet4hChangePercent: Option<f64>,
    // 6h
    pub uniqueWallet6h: Option<u64>,
    pub uniqueWalletHistory6h: Option<u64>,
    pub uniqueWallet6hChangePercent: Option<f64>,
    // 8h
    pub uniqueWallet8h: Option<u64>,
    pub uniqueWalletHistory8h: Option<u64>,
    pub uniqueWallet8hChangePercent: Option<f64>,
    // 12h
    pub uniqueWallet12h: Option<u64>,
    pub uniqueWalletHistory12h: Option<u64>,
    pub uniqueWallet12hChangePercent: Option<f64>,
    // 24h
    pub uniqueWallet24h: Option<u64>,
    pub uniqueWalletHistory24h: Option<u64>,
    pub uniqueWallet24hChangePercent: Option<f64>,

    pub lastTradeUnixTime: Option<i64>,
    pub lastTradeHumanTime: Option<String>,

    pub supply: Option<f64>,
    pub mc: Option<f64>,

    // 30min
    pub trade30m: Option<u64>,
    pub tradeHistory30m: Option<u64>,
    pub trade30mChangePercent: Option<f64>,
    pub sell30m: Option<u64>,
    pub sellHistory30m: Option<u64>,
    pub sell30mChangePercent: Option<f64>,
    pub buy30m: Option<u64>,
    pub buyHistory30m: Option<u64>,
    pub buy30mChangePercent: Option<f64>,
    pub v30m: Option<f64>,
    pub v30mUsd: Option<f64>,
    pub vHistory30m: Option<f64>,
    pub vHistory30mUsd: Option<f64>,
    pub v30mChangePercent: Option<f64>,
    pub vBuy30m: Option<f64>,
    pub vBuy30mUsd: Option<f64>,
    pub vBuyHistory30m: Option<f64>,
    pub vBuyHistory30mUsd: Option<f64>,
    pub vBuy30mChangePercent: Option<f64>,
    pub vSell30m: Option<f64>,
    pub vSell30mUsd: Option<f64>,
    pub vSellHistory30m: Option<f64>,
    pub vSellHistory30mUsd: Option<f64>,
    pub vSell30mChangePercent: Option<f64>,
    // 1h
    pub trade1h: Option<u64>,
    pub tradeHistory1h: Option<u64>,
    pub trade1hChangePercent: Option<f64>,
    pub sell1h: Option<u64>,
    pub sellHistory1h: Option<u64>,
    pub sell1hChangePercent: Option<f64>,
    pub buy1h: Option<u64>,
    pub buyHistory1h: Option<u64>,
    pub buy1hChangePercent: Option<f64>,
    pub v1h: Option<f64>,
    pub v1hUsd: Option<f64>,
    pub vHistory1h: Option<f64>,
    pub vHistory1hUsd: Option<f64>,
    pub v1hChangePercent: Option<f64>,
    pub vBuy1h: Option<f64>,
    pub vBuy1hUsd: Option<f64>,
    pub vBuyHistory1h: Option<f64>,
    pub vBuyHistory1hUsd: Option<f64>,
    pub vBuy1hChangePercent: Option<f64>,
    pub vSell1h: Option<f64>,
    pub vSell1hUsd: Option<f64>,
    pub vSellHistory1h: Option<f64>,
    pub vSellHistory1hUsd: Option<f64>,
    pub vSell1hChangePercent: Option<f64>,
    // 2h
    pub trade2h: Option<u64>,
    pub tradeHistory2h: Option<u64>,
    pub trade2hChangePercent: Option<f64>,
    pub sell2h: Option<u64>,
    pub sellHistory2h: Option<u64>,
    pub sell2hChangePercent: Option<f64>,
    pub buy2h: Option<u64>,
    pub buyHistory2h: Option<u64>,
    pub buy2hChangePercent: Option<f64>,
    pub v2h: Option<f64>,
    pub v2hUsd: Option<f64>,
    pub vHistory2h: Option<f64>,
    pub vHistory2hUsd: Option<f64>,
    pub v2hChangePercent: Option<f64>,
    pub vBuy2h: Option<f64>,
    pub vBuy2hUsd: Option<f64>,
    pub vBuyHistory2h: Option<f64>,
    pub vBuyHistory2hUsd: Option<f64>,
    pub vBuy2hChangePercent: Option<f64>,
    pub vSell2h: Option<f64>,
    pub vSell2hUsd: Option<f64>,
    pub vSellHistory2h: Option<f64>,
    pub vSellHistory2hUsd: Option<f64>,
    pub vSell2hChangePercent: Option<f64>,

    // 4h
    pub trade4h: Option<u64>,
    pub tradeHistory4h: Option<u64>,
    pub trade4hChangePercent: Option<f64>,
    pub sell4h: u64,
    pub sellHistory4h: Option<u64>,
    pub sell4hChangePercent: Option<f64>,
    pub buy4h: u64,
    pub buyHistory4h: Option<u64>,
    pub buy4hChangePercent: Option<f64>,
    pub v4h: f64,
    pub v4hUsd: f64,
    pub vHistory4h: Option<f64>,
    pub vHistory4hUsd: Option<f64>,
    pub v4hChangePercent: Option<f64>,
    pub vBuy4h: f64,
    pub vBuy4hUsd: Option<f64>,
    pub vBuyHistory4h: Option<f64>,
    pub vBuyHistory4hUsd: Option<f64>,
    pub vBuy4hChangePercent: Option<f64>,
    pub vSell4h: Option<f64>,
    pub vSell4hUsd: Option<f64>,
    pub vSellHistory4h: Option<f64>,
    pub vSellHistory4hUsd: Option<f64>,
    pub vSell4hChangePercent: Option<f64>,
    // 6h
    pub trade6h: Option<u64>,
    pub tradeHistory6h: Option<u64>,
    pub trade6hChangePercent: Option<f64>,
    pub sell6h: Option<u64>,
    pub sellHistory6h: Option<u64>,
    pub sell6hChangePercent: Option<f64>,
    pub buy6h: Option<u64>,
    pub buyHistory6h: Option<u64>,
    pub buy6hChangePercent: Option<f64>,
    pub v6h: Option<f64>,
    pub v6hUsd: Option<f64>,
    pub vHistory6h: Option<f64>,
    pub vHistory6hUsd: Option<f64>,
    pub v6hChangePercent: Option<f64>,
    pub vBuy6h: Option<f64>,
    pub vBuy6hUsd: Option<f64>,
    pub vBuyHistory6h: Option<f64>,
    pub vBuyHistory6hUsd: Option<f64>,
    pub vBuy6hChangePercent: Option<f64>,
    pub vSell6h: Option<f64>,
    pub vSell6hUsd: Option<f64>,
    pub vSellHistory6h: Option<f64>,
    pub vSellHistory6hUsd: Option<f64>,
    pub vSell6hChangePercent: Option<f64>,
    // 8h
    pub trade8h: Option<u64>,
    pub tradeHistory8h: Option<u64>,
    pub trade8hChangePercent: Option<f64>,
    pub sell8h: Option<u64>,
    pub sellHistory8h: Option<u64>,
    pub sell8hChangePercent: Option<f64>,
    pub buy8h: Option<u64>,
    pub buyHistory8h: Option<u64>,
    pub buy8hChangePercent: Option<f64>,
    pub v8h: Option<f64>,
    pub v8hUsd: Option<f64>,
    pub vHistory8h: Option<f64>,
    pub vHistory8hUsd: Option<f64>,
    pub v8hChangePercent: Option<f64>,
    pub vBuy8h: Option<f64>,
    pub vBuy8hUsd: Option<f64>,
    pub vBuyHistory8h: Option<f64>,
    pub vBuyHistory8hUsd: Option<f64>,
    pub vBuy8hChangePercent: Option<f64>,
    pub vSell8h: Option<f64>,
    pub vSell8hUsd: Option<f64>,
    pub vSellHistory8h: Option<f64>,
    pub vSellHistory8hUsd: Option<f64>,
    pub vSell8hChangePercent: Option<f64>,

    // 12h
    pub trade12h: Option<u64>,
    pub tradeHistory12h: Option<u64>,
    pub trade12hChangePercent: Option<f64>,
    pub sell12h: Option<u64>,
    pub sellHistory12h: Option<u64>,
    pub sell12hChangePercent: Option<f64>,
    pub buy12h: Option<u64>,
    pub buyHistory12h: Option<u64>,
    pub buy12hChangePercent: Option<f64>,
    pub v12h: Option<f64>,
    pub v12hUsd: Option<f64>,
    pub vHistory12h: Option<f64>,
    pub vHistory12hUsd: Option<f64>,
    pub v12hChangePercent: Option<f64>,
    pub vBuy12h: Option<f64>,
    pub vBuy12hUsd: Option<f64>,
    pub vBuyHistory12h: Option<f64>,
    pub vBuyHistory12hUsd: Option<f64>,
    pub vBuy12hChangePercent: Option<f64>,
    pub vSell12h: Option<f64>,
    pub vSell12hUsd: Option<f64>,
    pub vSellHistory12h: Option<f64>,
    pub vSellHistory12hUsd: Option<f64>,
    pub vSell12hChangePercent: Option<f64>,
    // 24h
    pub trade24h: Option<u64>,
    pub tradeHistory24h: Option<u64>,
    pub trade24hChangePercent: Option<f64>,
    pub sell24h: Option<u64>,
    pub sellHistory24h: Option<u64>,
    pub sell24hChangePercent: Option<f64>,
    pub buy24h: Option<u64>,
    pub buyHistory24h: Option<u64>,
    pub buy24hChangePercent: Option<f64>,
    pub v24h: Option<f64>,
    pub v24hUsd: Option<f64>,
    pub vHistory24h: Option<f64>,
    pub vHistory24hUsd: Option<f64>,
    pub v24hChangePercent: Option<f64>,
    pub vBuy24h: Option<f64>,
    pub vBuy24hUsd: Option<f64>,
    pub vBuyHistory24h: Option<f64>,
    pub vBuyHistory24hUsd: Option<f64>,
    pub vBuy24hChangePercent: Option<f64>,
    pub vSell24h: Option<f64>,
    pub vSell24hUsd: Option<f64>,
    pub vSellHistory24h: Option<f64>,
    pub vSellHistory24hUsd: Option<f64>,
    pub vSell24hChangePercent: Option<f64>,

    pub watch: Option<String>,

    // 30min view
    pub view30m: Option<u64>,
    pub viewHistory30m: Option<u64>,
    pub view30mChangePercent: Option<f64>,
    // 1h
    pub view1h: Option<u64>,
    pub viewHistory1h: Option<u64>,
    pub view1hChangePercent: Option<f64>,
    // 2h
    pub view2h: Option<u64>,
    pub viewHistory2h: Option<u64>,
    pub view2hChangePercent: Option<f64>,
    // 4h
    pub view4h: Option<u64>,
    pub viewHistory4h: Option<u64>,
    pub view4hChangePercent: Option<f64>,
    // 6h
    pub view6h: Option<u64>,
    pub viewHistory6h: Option<u64>,
    pub view6hChangePercent: Option<f64>,
    // 8h
    pub view8h: Option<u64>,
    pub viewHistory8h: Option<u64>,
    pub view8hChangePercent: Option<f64>,
    // 12h
    pub view12h: Option<u64>,
    pub viewHistory12h: Option<u64>,
    pub view12hChangePercent: Option<f64>,
    // 24h
    pub view24h: Option<u64>,
    pub viewHistory24h: Option<u64>,
    pub view24hChangePercent: Option<f64>,

    // 30min
    pub uniqueView30m: Option<u64>,
    pub uniqueViewHistory30m: Option<u64>,
    pub uniqueView30mChangePercent: Option<f64>,
    // 1h
    pub uniqueView1h: Option<u64>,
    pub uniqueViewHistory1h: Option<u64>,
    pub uniqueView1hChangePercent: Option<f64>,
    // 2h
    pub uniqueView2h: Option<u64>,
    pub uniqueViewHistory2h: Option<u64>,
    pub uniqueView2hChangePercent: Option<f64>,
    // 4h
    pub uniqueView4h: Option<u64>,
    pub uniqueViewHistory4h: Option<u64>,
    pub uniqueView4hChangePercent: Option<f64>,
    // 6h
    pub uniqueView6h: Option<u64>,
    pub uniqueViewHistory6h: Option<u64>,
    pub uniqueView6hChangePercent: Option<f64>,
    // 8h
    pub uniqueView8h: Option<u64>,
    pub uniqueViewHistory8h: Option<u64>,
    pub uniqueView8hChangePercent: Option<f64>,
    // 12h
    pub uniqueView12h: Option<u64>,
    pub uniqueViewHistory12h: Option<u64>,
    pub uniqueView12hChangePercent: Option<f64>,
    // 24h
    pub uniqueView24h: Option<u64>,
    pub uniqueViewHistory24h: Option<u64>,
    pub uniqueView24hChangePercent: Option<f64>,

    pub numberMarkets: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
//todo: test
pub struct Extensions {
    #[serde(flatten)]
    pub properties: HashMap<String, Option<String>>,
}
