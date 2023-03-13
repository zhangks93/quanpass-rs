use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use crate::trade::futures_client::{FuturesClient, Order};
use crate::util::number_util::{get_precision, round};
use crate::util::time_util::hours_ago_timestamp;
use binance::api::Binance;
use binance::market::Market;
use polars::export::num::ToPrimitive;

pub struct ShortLeaderStrategy {
    name: String,
    futures_client: FuturesClient,
    params: HashMap<String, f32>,
}

impl ShortLeaderStrategy {
    pub fn new(params: HashMap<String, f32>) -> ShortLeaderStrategy {
        ShortLeaderStrategy {
            name: String::from("强势做空策略"),
            futures_client: FuturesClient::new(),
            params: params,
        }
    }
}

impl Strategy for ShortLeaderStrategy {
    fn excute(&self) {
        // 1. get open orders and filter the orders updated 2 hours ago
        let orders = self.futures_client.open_orders().unwrap();
        let filtered = orders
            .into_iter()
            .filter(|item| item.update_time < hours_ago_timestamp(2))
            .collect::<Vec<Order>>();
        // 2. cancel the filtered order list
        for order in filtered {
            match self
                .futures_client
                .cancel_order(order.symbol, order.order_id)
            {
                Ok(_) => println!("Cancel Success"),
                Err(_) => println!("Cancel Failed"),
            }
        }
        // 3.filter top 5 futures and place an order with a price above 17% from the latest price
        let mut futures = self.futures_client.get_futures().unwrap();
        futures.sort_by(|a, b| {
            a.price_change_percent
                .total_cmp(&b.price_change_percent)
                .reverse()
        });
        for i in 1..5 {
            let temp = futures.get(i).unwrap();
           
            match self.futures_client.place_order(
                temp.symbol.to_owned(),
                "SELL".to_owned(),
                "SHORT".to_owned(),
                "LIMIT".to_owned(),
                round(temp.last_price * 1.17, get_precision(temp.last_price)),
                500.0,
            ) {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(_) => println!("Place Order Failed"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::strategy::strategy::Strategy;

    use super::ShortLeaderStrategy;

    #[test]
    fn test_excute() {
        ShortLeaderStrategy::new(HashMap::new()).excute()
    }
}
