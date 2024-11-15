use crate::{
    client::{
        binance_client::BinanceClient,
        binance_domain::{CurrentPrice, FutureTransaction, Kline, Transaction},
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

    pub fn change_leverage(&self, symbol: &str, leverage: u8) -> Result<Value> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = self.binance_client.build_signed_request(parameters);
        match self
            .binance_client
            .post_signed("/fapi/v1/leverage", Some(request))
        {
            Ok(response) => Ok(response),
            Err(err) => {
                println!("{:?}", err);
                bail!("Failed to change leverage")
            }
        }
    }

    pub fn current_price(&self, symbol: &str) -> Result<CurrentPrice, anyhow::Error> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = self.binance_client.build_request(parameters);

        match self
            .binance_client
            .get("/fapi/v1/ticker/price", Some(request))
        {
            Ok(current_price) => Ok(current_price),
            Err(err) => {
                println!("{:?}", err);
                bail!("Failed to get future current price")
            }
        }
    }

    pub fn kline(
        &self,
        symbol: &str,
        interval: &str,
        limit: Option<u16>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<Kline>, anyhow::Error> {
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
            .map_err(|err| {
                println!("{:?}", err);
                anyhow::anyhow!("Failed to get kline data")
            })?;
        println!("{:?}", data);

        let klines = data
            .iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<Kline>, _>>()
            .map_err(|err| {
                println!("{:?}", err);
                anyhow::anyhow!("Failed to parse kline data")
            })?;

        Ok(klines)
    }

    pub fn place_order(
        &self,
        symbol: String,
        side: String,
        order_type: String,
        price: f64,
        quantity: f64,
        stop_price: Option<f64>,
    ) -> Result<FutureTransaction, anyhow::Error> {
        let mut params: BTreeMap<String, _> = BTreeMap::new();
        params.insert("symbol".to_owned(), symbol);
        params.insert("side".to_owned(), side);
        params.insert("type".to_owned(), order_type);
        params.insert("price".to_owned(), price.to_string());
        params.insert("quantity".to_owned(), quantity.to_string());
        params.insert("timeInForce".to_owned(), "GTC".to_owned());
        if let Some(stop_price) = stop_price {
            params.insert("stopPrice".into(), stop_price.to_string());
        }
        let request = self
            .binance_client
            .build_future_signed_request(params, 5000);
        match self
            .binance_client
            .post_signed("/fapi/v1/order", Some(request))
        {
            Ok(transaction) => Ok(transaction),
            Err(err) => bail!("place open orders failed, {}", err),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::FutureClient;

    #[test]
    fn test_current_price() {
        let client = FutureClient::new();
        let result = client.current_price("BTCUSDT");
        assert!(result.is_ok());
        let current_price = result.unwrap();
        println!("Current price of BTCUSDT: {:?}", current_price);
        assert!(current_price.price > 0.0);
    }

    #[test]
    fn test_change_leverage() {
        let client = FutureClient::new();
        let symbol = "BTCUSDT";
        let leverage = 3;

        let result = client.change_leverage(symbol, leverage);
        match result {
            Ok(order_response) => {
                println!("Buy order response: {:?}", order_response);
            }
            Err(err) => {
                println!("Error placing buy order: {:?}", err);
            }
        }
    }

    #[test]
    fn test_kline() {
        let client = FutureClient::new();
        let symbol = "GALAUSDT";
        let interval = "1h";
        let limit = Some(10);
        let start_time = None;
        let end_time = None;

        let result = client.kline(symbol, interval, limit, start_time, end_time);
        assert!(result.is_ok());

        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert!(klines.len() <= 10); // Verify limit works

        // Print all klines
        klines.iter().for_each(|kline| {
            println!("Kline: {:?}", kline);
        });
    }

    #[test]
    fn test_limit_buy_with_stop_loss() {
        let client = FutureClient::new();
        let symbol = "BTCUSDT";
        let side = "SELL";
        let quantity = 0.01;
        let price = 78500.0;
        // Place STOP_LOSS order
        let stop_loss_price = 79000.0;
        let order_type = "STOP";

        let result = client.place_order(
            symbol.to_owned(),
            side.to_owned(),
            order_type.to_owned(),
            price,
            quantity,
            Some(stop_loss_price),
        );
        match result {
            Ok(order_response) => {
                println!("Buy order response: {:?}", order_response);
            }
            Err(err) => {
                println!("Error placing buy order: {:?}", err);
            }
        }
    }

    #[test]
    fn test_limit_buy() {
        let client = FutureClient::new();
        let symbol = "BTCUSDT";
        let quantity = 0.01;
        let price = 40000.0;

        let result = client.place_order(
            symbol.to_owned(),
            "BUY".to_owned(),
            "LIMIT".to_owned(),
            price,
            quantity,
            None,
        );
        match result {
            Ok(order_response) => {
                println!("Buy order response: {:?}", order_response);
            }
            Err(err) => {
                println!("Error placing buy order: {:?}", err);
            }
        }
    }
}
