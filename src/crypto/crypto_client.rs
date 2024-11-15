use std::collections::BTreeMap;

use serde_json::Value;

use crate::client::{
    binance_client::BinanceClient,
    binance_domain::{CurrentPrice, Kline, Order, Ticker, Transaction},
};

#[derive(Clone)]
pub struct CryptoClient {
    binance_client: BinanceClient,
}

impl CryptoClient {
    pub fn new() -> CryptoClient {
        CryptoClient {
            binance_client: BinanceClient::new(),
        }
    }

    pub fn limit_buy(
        &self,
        symbol: &str,
        quantity: f32,
        price: f64,
        client_order_id: &str,
    ) -> Transaction {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), symbol.to_owned());
        order_parameters.insert("side".into(), "BUY".to_owned());
        order_parameters.insert("timeInForce".into(), "GTC".to_owned());
        order_parameters.insert("quantity".into(), quantity.to_string());
        order_parameters.insert("type".into(), "LIMIT".to_owned());
        order_parameters.insert("price".into(), price.to_string());
        order_parameters.insert("newClientOrderId".into(), client_order_id.to_owned());
        let request = self.binance_client.build_signed_request(order_parameters);
        match self
            .binance_client
            .post_signed("/api/v3/order", Some(request))
        {
            Ok(_transaction) => return _transaction,
            Err(_err) => return Transaction::default(),
        }
    }

    pub fn limit_sell(
        &self,
        symbol: &str,
        quantity: f32,
        price: f64,
        client_order_id: &str,
    ) -> Transaction {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), symbol.to_owned());
        order_parameters.insert("side".into(), "SELL".to_owned());
        order_parameters.insert("timeInForce".into(), "GTC".to_owned());
        order_parameters.insert("quantity".into(), quantity.to_string());
        order_parameters.insert("type".into(), "LIMIT".to_owned());
        order_parameters.insert("price".into(), price.to_string());
        order_parameters.insert("newClientOrderId".into(), client_order_id.to_owned());
        let request = self.binance_client.build_signed_request(order_parameters);
        match self
            .binance_client
            .post_signed("/api/v3/order", Some(request))
        {
            Ok(_transaction) => return _transaction,
            Err(_err) => return Transaction::default(),
        }
    }

    pub fn open_orders(&self) -> Vec<Order> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("recvWindow".into(), "2000".to_owned());

        let request = self.binance_client.build_signed_request(parameters);
        return self
            .binance_client
            .get_signed("/api/v3/openOrders", Some(request))
            .unwrap();
    }

    pub fn current_price(&self, symbol: &str) -> CurrentPrice {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = self.binance_client.build_request(parameters);
        match self
            .binance_client
            .get("/api/v3/ticker/price", Some(request))
        {
            Ok(_price) => return _price,
            Err(_err) => {
                return CurrentPrice {
                    symbol: symbol.to_owned(),
                    price: 0.0,
                }
            }
        }
    }

    pub fn tickers(&self, ) -> Vec<Ticker> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let request = self.binance_client.build_request(parameters);
        match self
            .binance_client
            .get("/api/v3/ticker/24hr", Some(request))
        {
            Ok(_tickers) => return _tickers,
            Err(_err) => 
                return Vec::new()
        }
        
    }

    pub fn klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: Option<u16>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Vec<Kline> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        if let Some(lt) = limit {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        let request = self.binance_client.build_request(parameters);
        let data: Vec<Vec<Value>> = self
            .binance_client
            .get("/api/v3/klines", Some(request))
            .unwrap();

        return data
            .iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<Kline>, _>>()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::util::string_util::generate_random_id;

    use super::CryptoClient;

    #[test]
    fn test_limit_sell_and_limit_buy() {
        let client = CryptoClient::new();
        let transaction_buy = client.limit_buy("MANTAFDUSD", 10.0, 0.69, &generate_random_id());
        let transaction_sell = client.limit_sell("MANTAFDUSD", 10.0, 0.72, &generate_random_id());
        println!("{:?}", transaction_buy);
        println!("{:?}", transaction_sell);
    }

    #[test]
    fn test_get_current_price() {
        let client = CryptoClient::new();
        let result = client.current_price("PORTALFDUSD");
        println!("{:?}", result);
    }

    #[test]
    fn test_get_open_orders() {
        let client = CryptoClient::new();
        client.open_orders().iter().for_each(|order| {
            println!("{:?}", order);
        });
    }

    #[test]
    fn test_get_klines() {
        let client = CryptoClient::new();
        client
            .klines("GALAFDUSD", "1d", Some(30), None, None)
            .iter()
            .for_each(|kline| {
                println!("{:?}", kline);
            })
    }

    #[test]
    fn test_get_tickers() {
        let client = CryptoClient::new();
        client
            .tickers()
            .iter()
            .filter(|ticker| ticker.symbol.contains("FDUSD"))
            .for_each(|ticker| {
                println!("{:?}", ticker);
            })
    }
}
