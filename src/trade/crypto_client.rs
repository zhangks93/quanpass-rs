use binance::api::Binance;
use binance::account::Account;

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

    pub fn limit_buy(&self, symbol: &str, quantity: f32, price: f64) {
        match binance_client.limit_buy(symbol, quantity, price) {
            Ok(answer) => println!("{:?}", answer),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    
}