use anyhow::{bail, Result};
use binance::api::Binance;
use binance::futures::account::FuturesAccount;

pub struct FuturesClient {
    binance_client: FuturesAccount,
}

impl FuturesClient {
    pub fn new() -> FuturesClient {
        let api_key =
            Some("uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP".into());
        let secret_key =
            Some("LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB".into());
            FuturesClient {
            binance_client: Binance::new(api_key, secret_key),
        }
    }

    pub fn limit_buy(&self, symbol: &str, quantity: f32, price: f64) {
        match self.binance_client.limit_buy(symbol, quantity, price, binance::futures::account::TimeInForce::GTC) {
            Ok(answer) => println!("{:?}", answer),
            Err(e) => {println!("Error: {:?}", e); return },
        }
    }

}
