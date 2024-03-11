use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::futures_client::{FuturesClient, Order};
use crate::util::number_util::{get_precision, rand, round, smart_quantity};
use crate::util::time_util::{hours_ago_timestamp, minutes_ago_timestamp};
use rand::Rng;

pub struct ShortLeaderStrategy {
    name: String,
    futures_client: FuturesClient,
    params: HashMap<String, String>,
}

impl ShortLeaderStrategy {
    pub fn new(params: HashMap<String, String>) -> ShortLeaderStrategy {
        ShortLeaderStrategy {
            name: String::from("强势做空策略"),
            futures_client: FuturesClient::new(),
            params: params,
        }
    }
}

impl Clone for ShortLeaderStrategy {
    fn clone(&self) -> Self {
        ShortLeaderStrategy {
            name: self.name.clone(),
            futures_client: self.futures_client.clone(), // Ensure CryptoClient also implements Clone
            params: self.params.clone(),
        }
    }
}

impl Strategy for ShortLeaderStrategy {
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new((*self).clone())
    }

    fn excute(&self) {
        // 1.filter top 5 futures and place an order with a price above 23% from the latest price
        let order_size = 300.0;
        let mut futures = self.futures_client.get_futures().unwrap();

        futures.sort_by(|a, b| {
            a.price_change_percent
                .total_cmp(&b.price_change_percent)
                .reverse()
        });

        for i in (11..66).step_by(3) {
            let temp = futures.get(i).unwrap();
            let volatility = round((rand() * 3.5 + 1.5) / 100.0, 3);
            let limit_price_short = round(
                temp.last_price * (1.0 + volatility),
                get_precision(temp.last_price),
            );
            let limit_price_long = round(
                temp.last_price * (1.0 - volatility),
                get_precision(temp.last_price),
            );
            println!("{}", temp.symbol);
            println!("{}", temp.last_price);
            println!("{}", limit_price_short);
            println!("{}", limit_price_long);
            let mut rng = rand::thread_rng();
            match self
                .futures_client
                .change_margin_type(temp.symbol.to_owned(), "ISOLATED".to_owned())
            {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(_) => {
                    println!("Change MarginType Failed");
                    continue;
                }
            }
            if rng.gen_range(0..10) % 2 == 0 {
                match self.futures_client.place_order(
                    temp.symbol.to_owned(),
                    "SELL".to_owned(),
                    "SHORT".to_owned(),
                    "LIMIT".to_owned(),
                    limit_price_short,
                    smart_quantity(order_size, limit_price_short),
                ) {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(_) => println!("Place Order Failed"),
                }
            } else {
                match self.futures_client.place_order(
                    temp.symbol.to_owned(),
                    "BUY".to_owned(),
                    "LONG".to_owned(),
                    "LIMIT".to_owned(),
                    limit_price_long,
                    smart_quantity(order_size, limit_price_long),
                ) {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(_) => println!("Place Order Failed"),
                }
            }
        }

        // 2. get open orders and filter the orders updated 2 hours ago
        std::thread::sleep(std::time::Duration::from_secs(60));
        let orders = self.futures_client.open_orders().unwrap();
        let filtered = orders
            .into_iter()
            .filter(|item| item.update_time < minutes_ago_timestamp(30))
            .collect::<Vec<Order>>();
        // 3. cancel the filtered order list
        for order in filtered {
            match self
                .futures_client
                .cancel_order(order.symbol, order.order_id)
            {
                Ok(_) => println!("Cancel Success"),
                Err(_) => println!("Cancel Failed"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use binance::{api::Binance, market::Market};

    #[test]
    fn test_excute() {
        let market: Market = Binance::new(None, None);
        match market.get_all_24h_price_stats() {
            Ok(prices) => {
                let mut filtered_prices = prices
                    .iter() // or into_iter() if you want to consume the original vector
                    .filter(|price| price.symbol.ends_with("FDUSD"))
                    .cloned() // Clone the filtered items to a new collection if necessary
                    .collect::<Vec<_>>();
                filtered_prices.sort_by(|a, b| {
                    a.price_change_percent
                        .partial_cmp(&b.price_change_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                filtered_prices.iter().for_each(|price| {
                    println!(
                        "Symbol: {}, Percent: {}, Open: {}, High: {}, Low: {}, Close: {}",
                        price.clone().symbol,
                        price.clone().price_change_percent,
                        price.clone().open_price,
                        price.clone().high_price,
                        price.clone().low_price,
                        price.clone().last_price
                    )
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
