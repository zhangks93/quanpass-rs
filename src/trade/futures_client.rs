use crate::util::json_util::string_or_float;
use std::collections::BTreeMap;

use crate::client::binance::Binance;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    order_id: i64,
    symbol: String,
    status: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    time_in_force: String,
    side: String,
    position_side: String,
    updateTime: i64,
}

pub struct FuturesClient {
    binance: Binance,
}

impl FuturesClient {
    pub fn new() -> FuturesClient {
        FuturesClient {
            binance: Binance::new(),
        }
    }

    pub fn change_leverage(&self, symbol: String, leverage: i8) -> Result<String> {
        let mut params: BTreeMap<String, _> = BTreeMap::new();
        params.insert("symbol".to_owned(), symbol);
        params.insert("leverage".to_owned(), leverage.to_string());
        let request = self.binance.build_signed_request(params, 1000);
        match self
            .binance
            .post_signed("/fapi/v1/leverage".to_owned(), request)
        {
            Ok(resp) => Ok(resp),
            Err(err) => bail!("place open orders failed, {}", err),
        }
    }

    pub fn open_orders(&self) -> Result<Vec<Order>> {
        let request = self.binance.build_signed_request(BTreeMap::new(), 1000);
        match self
            .binance
            .get_signed("/fapi/v1/openOrders".to_owned(), Some(request))
        {
            Ok(resp) => {
                let result: Vec<Order> = serde_json::from_str(&resp).unwrap();
                Ok(result)
            }
            Err(err) => bail!("get open orders failed, {}", err),
        }
    }

    pub fn cancel_order(&self, symbol: String, order_id: i64) -> Result<String> {
        let mut params: BTreeMap<String, _> = BTreeMap::new();
        params.insert("symbol".to_owned(), symbol);
        params.insert("orderId".to_owned(), order_id.to_string());
        let request = self.binance.build_signed_request(params, 1000);
        match self
            .binance
            .delete_signed("/fapi/v1/order".to_owned(), Some(request))
        {
            Ok(resp) => Ok(resp),
            Err(err) => bail!("cancel open orders failed, {}", err),
        }
    }

    pub fn place_order(
        &self,
        symbol: String,
        side: String,
        position_side: String,
        order_type: String,
        price: f64,
        quantity: f64,
    ) -> Result<String> {
        let mut params: BTreeMap<String, _> = BTreeMap::new();
        params.insert("symbol".to_owned(), symbol);
        params.insert("side".to_owned(), side);
        params.insert("positionSide".to_owned(), position_side);
        params.insert("type".to_owned(), order_type);
        params.insert("price".to_owned(), price.to_string());
        params.insert("quantity".to_owned(), quantity.to_string());
        params.insert("timeInForce".to_owned(), "GTC".to_owned());
        let request = self.binance.build_signed_request(params, 1000);
        match self
            .binance
            .post_signed("/fapi/v1/order".to_owned(), request)
        {
            Ok(resp) => Ok(resp),
            Err(err) => bail!("place open orders failed, {}", err),
        }
    }
}

mod tests {
    use super::FuturesClient;

    #[test]
    fn test_open_orders() {
        let client: FuturesClient = FuturesClient::new();
        let orders = client.open_orders();
        println!("{:?}", orders.unwrap());
    }

    #[test]
    fn test_cancel_order() {
        let client: FuturesClient = FuturesClient::new();
        let order = client.cancel_order("AGIXUSDT".to_owned(), 169276916);
        println!("{:?}", order.unwrap());
    }

    #[test]
    fn test_place_order() {
        let client: FuturesClient = FuturesClient::new();
        let order = client.place_order(
            "AGIXUSDT".to_owned(),
            "BUY".to_owned(),
            "LONG".to_owned(),
            "LIMIT".to_owned(),
            0.28,
            500.0,
        );
        println!("{:?}", order.unwrap());
    }

    #[test]
    fn test_change_leverage() {
        let client: FuturesClient = FuturesClient::new();
        let order = client.change_leverage("AGIXUSDT".to_owned(), 10);
        println!("{:?}", order.unwrap());
    }
}
