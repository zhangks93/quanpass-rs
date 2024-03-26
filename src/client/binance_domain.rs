use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::util::json_util::string_or_float;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub iceberg_qty: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: Option<i64>,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
}

fn get_value(row: &[Value], index: usize, name: &'static str) -> Result<Value, ErrorKind> {
    Ok(row
        .get(index)
        .ok_or_else(|| ErrorKind::InvalidData)?
        .clone())
}
impl TryFrom<&Vec<Value>> for Kline {
    type Error = ErrorKind;

    fn try_from(row: &Vec<Value>) -> Result<Self, Self::Error> {
        Ok(Self {
            open_time: from_value(get_value(row, 0, "open_time")?).unwrap(),
            open: from_value(get_value(row, 1, "open")?).unwrap(),
            high: from_value(get_value(row, 2, "high").unwrap()).unwrap(),
            low: from_value(get_value(row, 3, "low").unwrap()).unwrap(),
            close: from_value(get_value(row, 4, "close").unwrap()).unwrap(),
            volume: from_value(get_value(row, 5, "volume").unwrap()).unwrap(),
            close_time: from_value(get_value(row, 6, "close_time").unwrap()).unwrap(),
            quote_asset_volume: from_value(get_value(row, 7, "quote_asset_volume").unwrap())
                .unwrap(),
            number_of_trades: from_value(get_value(row, 8, "number_of_trades").unwrap()).unwrap(),
            taker_buy_base_asset_volume: from_value(
                get_value(row, 9, "taker_buy_base_asset_volume").unwrap(),
            )
            .unwrap(),
            taker_buy_quote_asset_volume: from_value(
                get_value(row, 10, "taker_buy_quote_asset_volume").unwrap(),
            )
            .unwrap(),
        })
    }
}

fn default_stop_price() -> f64 {
    0.0
}
