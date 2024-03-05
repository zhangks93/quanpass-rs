use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use crate::util::number_util::{get_precision, round};
use binance::api::Binance;
use binance::market::Market;

pub struct GridStrategy {
    name: String,
    crypto_client: CryptoClient,
    params: HashMap<String, String>,
}

impl GridStrategy {
    pub fn new(params: HashMap<String, String>) -> GridStrategy {
        GridStrategy {
            name: String::from("现货网格策略"),
            crypto_client: CryptoClient::new(),
            params: params,
        }
    }
}

impl Clone for GridStrategy {
    fn clone(&self) -> Self {
        GridStrategy {
            name: self.name.clone(),
            crypto_client: self.crypto_client.clone(),
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
        let gap: f64 = self
            .params
            .get("gap")
            .unwrap()
            .parse()
            .expect("Not a valid f64");
        let quantity: f32 = self
            .params
            .get("quantity")
            .unwrap()
            .parse()
            .expect("Not a valid f32");
        let symbol = self.params.get("symbol").unwrap();
        let mut current_price = 0.0;

        match market.get_price(symbol.as_str()) {
            Ok(result) => current_price = result.price,
            Err(_) => {
                println!("Network Error");
                return;
            }
        }

        self.crypto_client.limit_buy(
            symbol.as_str(),
            quantity,
            round(
                (current_price * (1.0 - gap) * 100000.0).round() / 100000.0,
                get_precision(current_price),
            ),
        );
        self.crypto_client.limit_sell(
            symbol.as_str(),
            quantity,
            round(
                (current_price * (1.0 + gap) * 100000.0).round() / 100000.0,
                get_precision(current_price),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use binance::{api::Binance, market::Market, model::KlineSummary};

    #[test]
    fn test_excute() {
        let market: Market = Binance::new(None, None);
        match market.get_klines("GALAFDUSD", "1d", 30, None, None) {
            Ok(klines) => match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    for kilne in &klines {
                        println!(
                            "Open: {}, High: {}, Low: {}, Close: {}",
                            kilne.clone().open,
                            kilne.clone().high,
                            kilne.clone().low,
                            kilne.clone().close
                        )
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
