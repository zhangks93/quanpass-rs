use std::collections::HashMap;
use binance::model::Order;
use chrono::{Utc};

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
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
            name: String::from("网格策略"),
            crypto_client: CryptoClient::new(),
            symbol: symbol,
            params: params,
        }
    }
}

impl Strategy for GridStrategy {
    fn excute(&self) {
        println!("EXCUTION BEGIN!");
        let market: Market = Binance::new(None, None);
        let gap = self.params.get("gap").unwrap().to_f64().unwrap();
        let quantity = self.params.get("quantity").unwrap();
        let mut current_price = 0.0;

        match market.get_price(self.symbol.as_str()) {
            Ok(result) => current_price = result.price,
            Err(_) => {println!("Network Error"); return} 
        }

        let order_list = self.crypto_client.open_orders();
        
        // clear open orders created 2 hours ago
        // todo
        // order_list.iter().for_each(|e| println!("{:?}", e));

        let buy_result = self.crypto_client.limit_buy(
            self.symbol.as_str(),
            *quantity,
            (current_price * (1.0 - gap) * 100000.0).round() / 100000.0,
        );
        let sell_result = self.crypto_client.limit_sell(
            self.symbol.as_str(),
            *quantity,
            (current_price * (1.0 + gap) * 100000.0).round() / 100000.0,
        );

        println!(" 【{}】Price is {}, buy result is  {} and sell result is  {}", Utc::now(), current_price, buy_result, sell_result);
    }
}
