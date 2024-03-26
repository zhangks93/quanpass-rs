use crate::util::json_util::string_or_float;
use std::collections::BTreeMap;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: i64,
    pub symbol: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub time_in_force: String,
    pub side: String,
    pub position_side: String,
    pub update_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price_change_percent: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
}
