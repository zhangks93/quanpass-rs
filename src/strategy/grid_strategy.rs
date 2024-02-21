use binance::model::Order;
use chrono::Utc;
use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use crate::util::number_util::{get_precision, round};
use binance::api::Binance;
use binance::market::Market;
use polars::export::num::ToPrimitive;

pub struct GridStrategy {
    name: String,
    crypto_client: CryptoClient,
    symbol: String,
    params: HashMap<String, f32>,
}

impl GridStrategy {
    pub fn new(symbol: String, params: HashMap<String, f32>) -> GridStrategy {
        GridStrategy {
            name: String::from("现货网格策略"),
            crypto_client: CryptoClient::new(),
            symbol: symbol,
            params: params,
        }
    }
}

impl Clone for GridStrategy {
    fn clone(&self) -> Self {
        GridStrategy {
            name: self.name.clone(),
            crypto_client: self.crypto_client.clone(), // Ensure CryptoClient also implements Clone
            symbol: self.symbol.clone(),
            params: self.params.clone(),
        }
    }
}

impl Strategy for GridStrategy {
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new((*self).clone())
    }

    fn excute(&self) {
        let market: Market = Binance::new(None, None);
        let gap = self.params.get("gap").unwrap().to_f64().unwrap();
        let quantity = self.params.get("quantity").unwrap();
        let mut current_price = 0.0;

        match market.get_price(self.symbol.as_str()) {
            Ok(result) => current_price = result.price,
            Err(_) => {
                println!("Network Error");
                return;
            }
        }

        self.crypto_client.limit_buy(
            self.symbol.as_str(),
            *quantity,
            round((current_price * (1.0 - gap) * 100000.0).round() / 100000.0, get_precision(current_price)),
        );
        self.crypto_client.limit_sell(
            self.symbol.as_str(),
            *quantity,
            round((current_price * (1.0 + gap) * 100000.0).round() / 100000.0, get_precision(current_price)),
        );
    }
}
