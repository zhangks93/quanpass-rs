use binance::model::Order;
use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use binance::api::Binance;
use binance::market::Market;
use polars::export::num::ToPrimitive;

pub struct TriangleStrategy {
    name: String,
    crypto_client: CryptoClient,
    symbol: String,
    params: HashMap<String, f32>,
}

impl TriangleStrategy {
    pub fn new(symbol: String, params: HashMap<String, f32>) -> TriangleStrategy {
        TriangleStrategy {
            name: String::from("三角策略"),
            crypto_client: CryptoClient::new(),
            symbol: symbol,
            params: params,
        }
    }
}

impl Strategy for TriangleStrategy {
    fn excute(& self) {
        let market: Market = Binance::new(None, None);
        let gap = self.params.get("gap").unwrap().to_f64().unwrap();
        let quantity = self.params.get("quantity").unwrap();
        let splited: Vec<&str> =  self.symbol.split(';').collect();
        let symbol_1 = String::from(splited[0]);
        let symbol_2 = String::from(splited[1]);
        let mut current_price_1 = 0.0;
        let mut current_price_2 = 0.0;
        // get the latest price
        match market.get_price(symbol_1.as_str()) {
            Ok(result) => current_price_1 = result.price,
            Err(_) => {
                println!("Network Error");
                return;
            }
        }
        match market.get_price(symbol_2.as_str()) {
            Ok(result) => current_price_2 = result.price,
            Err(_) => {
                println!("Network Error");
                return;
            }
        }
        // cancel orders which are expired
        /* 
        let open_orders: Vec<Order> = self.crypto_client.open_orders();
        open_orders
            .iter()
            .filter(|order| order.time > 10000000)
            .for_each(|order| {
                self.crypto_client
                    .cancel_order(order.symbol.to_string(), order.order_id);
            });
        */
        // make orders
        self.crypto_client.limit_buy(
            symbol_1.as_str(),
            *quantity,
            (current_price_1 * (1.0 - gap) * 100000.0).round() / 100000.0,
        );
        self.crypto_client.limit_sell(
            symbol_2.as_str(),
            *quantity,
            (current_price_2 * (1.0 + gap) * 100000.0).round() / 100000.0,
        );
    }
}
