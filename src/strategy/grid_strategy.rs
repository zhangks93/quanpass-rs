use binance::model::Order;
use chrono::Utc;
use std::collections::HashMap;

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
            name: String::from("现货网格策略"),
            crypto_client: CryptoClient::new(),
            symbol: symbol,
            params: params,
        }
    }
}

impl Strategy for GridStrategy {
    fn excute(& self) {
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
        // cancel orders which are expired
        let open_orders: Vec<Order> = self.crypto_client.open_orders();
        open_orders
            .iter()
            .filter(|order| order.time > 10000000)
            .for_each(|order| {
                self.crypto_client
                    .cancel_order(order.symbol.to_string(), order.order_id);
            });

        self.crypto_client.limit_buy(
            self.symbol.as_str(),
            *quantity,
            (current_price * (1.0 - gap) * 100000.0).round() / 100000.0,
        );
        self.crypto_client.limit_sell(
            self.symbol.as_str(),
            *quantity,
            (current_price * (1.0 + gap) * 100000.0).round() / 100000.0,
        );
    }
}
