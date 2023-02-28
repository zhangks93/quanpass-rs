use crate::strategy::grid_strategy::GridStrategy;

pub trait Strategy {
    fn excute(&self);
}

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn new() -> Self {
        StrategyFactory
    }

    pub fn create_strategy(&self, label: &str) -> Box<dyn Strategy> {
        if label == "Grid" {
            Box::new(GridStrategy::new())
        } else {
            Box::new(GridStrategy::new())
        }
    }
}
