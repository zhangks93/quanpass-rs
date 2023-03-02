use serde::Deserialize;
use error_chain::error_chain;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

