use anyhow::{bail, Result};
use binance::api::Binance;
use binance::futures::account::FuturesAccount;
use binance::account::Account;

pub struct ContractClient {
    binance_client: FuturesAccount,
}

impl ContractClient {
    pub fn new() -> ContractClient {
        let api_key =
            Some("uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP".into());
        let secret_key =
            Some("LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB".into());
        ContractClient {
            binance_client: Binance::new(api_key, secret_key),
        }
    }

    pub fn get_all_open_orders(&self, symbol: &str) {
        let api_key = Some("uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP".into());
    let secret_key = Some("LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }
    }
}
