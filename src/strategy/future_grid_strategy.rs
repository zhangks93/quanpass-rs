use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::crypto::futures_client::FutureClient;
use crate::strategy::strategy::Strategy;
use crate::util::number_util::{get_precision, round};
use crate::util::string_util::generate_random_id;


#[derive(Clone)]
pub struct FutureGridStrategy {
    id: String,
    name: String,
    crypto_client: FutureClient,
    params: HashMap<String, String>,
}

impl FutureGridStrategy {
    pub fn new(params: HashMap<String, String>) -> Self {
        Self {
            id: generate_random_id(),
            name: String::from("FutureGrid"),
            crypto_client: FutureClient::new(),
            params,
        }
    }
}

impl Strategy for FutureGridStrategy {
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("name".to_string(), Value::String(self.name.clone()));
        
        let mut params_map = Map::new();
        for (key, value) in &self.params {
            params_map.insert(key.clone(), Value::String(value.clone()));
        }
        map.insert("params".to_string(), Value::Object(params_map));
        map
    }

    

    fn excute(&self) {
        let gap: f64 = self
            .params
            .get("gap")
            .unwrap()
            .parse()
            .expect("Not a valid f64");
        let quantity: f64 = self
            .params
            .get("quantity")
            .unwrap()
            .parse()
            .expect("Not a valid f64");
        let symbol = self.params.get("symbol").unwrap();
        let leverage: u8 = self
            .params
            .get("leverage")
            .unwrap()
            .parse()
            .expect("Not a valid i32");

        // Set leverage first
        if let Err(e) = self.crypto_client.change_leverage(symbol, leverage) {
            println!("Failed to set leverage: {}", e);
            return;
        }

        let current_price = match self.crypto_client.current_price(symbol) {
            Ok(price) => price.price,
            Err(e) => {
                println!("Failed to get current price: {}", e);
                return;
            }
        };

        // Place long position order
        let buy_price = round(
            (current_price * (1.0 - gap) * 100000000.0).round() / 100000000.0,
            get_precision(current_price),
        );
        
        if let Err(e) = self.crypto_client.place_order(
            symbol.to_string(),
            "BUY".to_string(),
            "LIMIT".to_string(),
            buy_price,
            quantity,
            None,
        ) {
            println!("Failed to place buy order: {}", e);
        }

        // Place short position order
        let sell_price = round(
            (current_price * (1.0 + gap) * 100000000.0).round() / 100000000.0,
            get_precision(current_price),
        );

        if let Err(e) = self.crypto_client.place_order(
            symbol.to_string(),
            "SELL".to_string(),
            "LIMIT".to_string(),
            sell_price,
            quantity,
            None,
        ) {
            println!("Failed to place sell order: {}", e);
        }
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }

    fn params(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_future_grid_strategy() {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("gap".into(), "0.003".to_string());
        parameters.insert("quantity".into(), "400".to_string());
        parameters.insert("symbol".into(), "XAIUSDT".to_string());
        parameters.insert("leverage".into(), "10".to_string());

        let strategy = FutureGridStrategy::new(parameters);
        strategy.excute();
    }
}
