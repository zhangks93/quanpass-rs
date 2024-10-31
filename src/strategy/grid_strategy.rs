use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::crypto::crypto_client::CryptoClient;
use crate::util::number_util::{get_precision, round};
use crate::util::string_util::generate_random_id;

pub struct GridStrategy {
    id: String,
    name: String,
    crypto_client: CryptoClient,
    params: HashMap<String, String>,
}

impl GridStrategy {
    pub fn new(params: HashMap<String, String>) -> GridStrategy {
        GridStrategy {
            id: generate_random_id(),
            name: String::from("现货网格策略"),
            crypto_client: CryptoClient::new(),
            params: params,
        }
    }
}

impl Clone for GridStrategy {
    fn clone(&self) -> Self {
        GridStrategy {
            id: self.id.clone(),
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
        let current_price = self.crypto_client.current_price(symbol.as_str()).price;

        self.crypto_client.limit_buy(
            symbol.as_str(),
            quantity,
            round(
                (current_price * (1.0 - gap) * 100000000.0).round() / 100000000.0,
                get_precision(current_price),
            ),
            self.id.as_str()
        );
        self.crypto_client.limit_sell(
            symbol.as_str(),
            quantity,
            round(
                (current_price * (1.0 + gap) * 100000000.0).round() / 100000000.0,
                get_precision(current_price),
            ),
            self.id.as_str()
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use binance::{api::Binance, market::Market};

    use crate::strategy::strategy::Strategy;

    use super::GridStrategy;

    #[test]
    fn test_get_klines() {
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

    #[test]
    fn test_excute() {
        let mut parameters: HashMap<String, String> = HashMap::new();

        parameters.insert("gap".into(), "0.005".to_string());
        parameters.insert("quantity".into(), "10.0".to_string());
        parameters.insert("symbol".into(), "ATOMFDUSD".to_string());
        let strategy = GridStrategy::new(parameters);
        strategy.excute();
    }
}
