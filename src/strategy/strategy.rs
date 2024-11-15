use serde_json::{Map, Value};

use crate::strategy::grid_strategy::GridStrategy;
use std::collections::HashMap;

use super::future_grid_strategy::FutureGridStrategy;

pub trait Strategy {
    fn name(&self) -> String;
    fn params(&self) -> HashMap<String, String>;
    fn excute(&self);
    fn clone_box(&self) -> Box<dyn Strategy>;
    fn to_json(&self) -> Map<String, Value>;
    
}

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn new() -> Self {
        StrategyFactory
    }

    

    pub fn create_strategy(
        &self,
        label: &str,
        params: HashMap<String, String>,
    ) -> Box<dyn Strategy> {
        match label {
            "Grid" => Box::new(GridStrategy::new(params)),
            "FutureGrid" => Box::new(FutureGridStrategy::new(params)),
            _ => Box::new(GridStrategy::new(params)),
        }
    }
}
