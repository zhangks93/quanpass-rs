use super::short_leader_strategy::ShortLeaderStrategy;
use crate::strategy::grid_strategy::GridStrategy;
use std::collections::HashMap;


pub trait Strategy {
    fn excute(&self);
    fn clone_box(&self) -> Box<dyn Strategy>;
    
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
        if label == "Grid" {
            Box::new(GridStrategy::new(params))
        } else if label == "Short Leader" {
            Box::new(ShortLeaderStrategy::new(params))
        } else {
            Box::new(GridStrategy::new(params))
        }
    }
}
