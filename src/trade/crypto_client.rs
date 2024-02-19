use binance::account::Account;
use binance::api::Binance;

pub struct CryptoClient {
    binance_client: Account,
}

impl CryptoClient {
    pub fn new() -> CryptoClient {
        let api_key =
            Some("uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP".into());
        let secret_key =
            Some("LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB".into());
        CryptoClient {
            binance_client: Binance::new(api_key, secret_key),
        }
    }

    pub fn limit_buy(&self, symbol: &str, quantity: f32, price: f64) -> bool {
        match self.binance_client.limit_buy(symbol, quantity, price) {
            Ok(_) => return true,
            Err(e) => {
                println!("Error: {:?}", e);
                return false;
            }
        }
    }

    pub fn limit_sell(&self, symbol: &str, quantity: f32, price: f64) -> bool {
        match self.binance_client.limit_sell(symbol, quantity, price) {
            Ok(_) => return true,
            Err(e) => {
                println!("Error: {:?}", e);
                return false;
            }
        }
    }

    pub fn open_orders(&self) -> Vec<binance::model::Order> {
        match self.binance_client.get_all_open_orders() {
            Ok(result) => result,
            Err(_) => Vec::new(),
        }
    }

    pub fn cancel_order(&self, symbol: String, order_id: u64) -> bool {
        match self.binance_client.cancel_order(symbol, order_id) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
