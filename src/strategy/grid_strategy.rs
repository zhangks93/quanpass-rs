use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use binance::api::Binance;
use binance::market::Market;
use futures::executor::block_on;
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
        let market: Market = Binance::new(None, None);
        let gap = self.params.get("gap").unwrap().to_f64().unwrap();
        let quantity = self.params.get("quantity").unwrap();


        let current_price = market.get_price(self.symbol.as_str()).unwrap().price;
        
        self.crypto_client.limit_buy(self.symbol.as_str(), *quantity, (current_price * (1.0 - gap) * 100000.0).round() / 100000.0);
        self.crypto_client.limit_sell(self.symbol.as_str(), *quantity, (current_price * (1.0 + gap) * 100000.0).round() / 100000.0);


        println!(" {:?}", current_price);
    }
}
