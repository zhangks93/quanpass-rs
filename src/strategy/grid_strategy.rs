use crate::market::crypto_client::CryptoClient;
use crate::strategy::strategy::Strategy;
use futures::executor::block_on;

pub struct GridStrategy {
    name: String,
    crypto_client: CryptoClient,
}

impl GridStrategy {
    pub fn new() -> GridStrategy {
        GridStrategy {
            name: String::from("网格策略"),
            crypto_client: CryptoClient::new(),
        }
    }
}

impl Strategy for GridStrategy {
    fn excute(&self) {
        
        let data = block_on(self.crypto_client.get_market("volume", "desc"));
        
        println!(" {:?}", data.unwrap());
    }
}
