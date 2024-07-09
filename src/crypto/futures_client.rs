use crate::{
    client::{
        binance_client::BinanceClient,
        binance_domain::{CurrentPrice, Kline, Transaction},
    },
    util::json_util::string_or_float,
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

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

#[derive(Clone)]
pub struct FutureClient {
    binance_client: BinanceClient,
}

impl FutureClient {
    pub fn new() -> FutureClient {
        FutureClient {
            binance_client: BinanceClient::new_with_host("https://fapi.binance.com"),
        }
    }

    pub fn current_price(&self, symbol: &str) -> CurrentPrice {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = self.binance_client.build_request(parameters);
        return self
            .binance_client
            .get("/fapi/v1/ticker/price", Some(request))
            .unwrap();
    }

    pub fn klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: Option<u16>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Vec<Kline> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        if let Some(lt) = limit {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        let request = self.binance_client.build_request(parameters);
        let data: Vec<Vec<Value>> = self
            .binance_client
            .get("/fapi/v1/klines", Some(request))
            .unwrap();
        println!("{:?}", data);

        return data
            .iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<Kline>, _>>()
            .unwrap();
    }

    pub fn limit_buy(&self, symbol: &str, quantity: f32, price: f64) -> Transaction {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), symbol.to_owned());
        order_parameters.insert("side".into(), "BUY".to_owned());
        order_parameters.insert("timeInForce".into(), "GTC".to_owned());
        order_parameters.insert("quantity".into(), quantity.to_string());
        order_parameters.insert("type".into(), "LIMIT".to_owned());
        order_parameters.insert("price".into(), price.to_string());
        let request = self.binance_client.build_signed_request(order_parameters);
        match self
            .binance_client
            .post_signed("/api/v3/order", Some(request))
        {
            Ok(_transaction) => {println!("{:?}", _transaction);return _transaction;},
            Err(_err) => {println!("{:?}", _err);return Transaction::default();},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FutureClient;

    #[test]
    fn test_get_klines() {
        let client = FutureClient::new();
        client
            .klines("GALAUSDT", "1d", Some(30), None, None)
            .iter()
            .for_each(|kline| {
                println!("{:?}", kline);
            })
    }

    #[test]
    fn test_get_current_price() {
        let client = FutureClient::new();
        let result = client.current_price("PORTALUSDT");
        println!("{:?}", result);
    }

    #[test]
    fn test_limit_sell_and_limit_buy() {
        let client = FutureClient::new();
        client.limit_buy("CFXUSDT", 20.0, 0.1);
    }
}
