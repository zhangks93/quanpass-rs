use std::collections::HashMap;
use crate::strategy::grid_strategy::GridStrategy;
use crate::strategy::triangle_strategy::TriangleStrategy;
use super::short_leader_strategy::ShortLeaderStrategy;

pub trait Strategy {
    fn excute(& self);
}

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn new() -> Self {
        StrategyFactory
    }

    pub fn create_strategy(&self, label: &str, symbol: String, params: HashMap<String, f32>) -> Box<dyn Strategy> {
        if label == "Grid" {
            Box::new(GridStrategy::new(symbol, params))
        } else if label == "Short Leader" {
            Box::new(ShortLeaderStrategy::new(params))
        } else if label == "Triangle" {
            Box::new(TriangleStrategy::new(symbol,params))
        } else {
            Box::new(GridStrategy::new(symbol, params))
        }
    }
}
