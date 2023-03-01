use std::collections::HashMap;

use crate::strategy::strategy::Strategy;
use crate::trade::crypto_client::CryptoClient;
use crate::trade::futures_client::FuturesClient;
use binance::api::Binance;
use binance::market::Market;
use polars::export::num::ToPrimitive;

pub struct ShortLeaderStrategy {
    name: String,
    crypto_client: CryptoClient,
    futures_client: FuturesClient,
    symbol: String,
    params: HashMap<String, f32>,
}

impl ShortLeaderStrategy {
    pub fn new(symbol: String, params: HashMap<String, f32>) -> ShortLeaderStrategy {
        ShortLeaderStrategy {
            name: String::from("强势做空策略"),
            crypto_client: CryptoClient::new(),
            futures_client: FuturesClient::new(),
            symbol: symbol,
            params: params,
        }
    }
}

impl Strategy for ShortLeaderStrategy {
    fn excute(&self) {
        let market: Market = Binance::new(None, None);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::strategy::strategy::Strategy;

    use super::ShortLeaderStrategy;

    #[test]
    fn test_excute() {
        ShortLeaderStrategy::new(String::from("demo"), HashMap::new()).excute()
    }
}
