use crate::strategy::strategy::{Strategy, StrategyFactory};
use job_scheduler::{Job, JobScheduler};
use crate::util::string_util::generate_random_id;
use once_cell::sync::Lazy;
use std::{sync::Mutex, collections::HashMap};

pub static mut MANAGER: Lazy<Mutex<JobScheduler>> = Lazy::new(|| Mutex::new(JobScheduler::new()));

pub struct Robot {
    id: String,
    name: String,
    strategy: Box<dyn Strategy>,
}

impl Robot {
    pub fn new(name: String, strategy: String, symbol: String, params: HashMap<String, f32>) -> Robot {
        Robot {
            id: generate_random_id(),
            name: name,
            strategy: StrategyFactory::new().create_strategy(&strategy, symbol, params),
        }
    }

    pub fn excute(&self) {
        self.strategy.excute();
    }

    pub fn append(robot: Robot, schedule: &str) {
        unsafe {
            MANAGER
                .lock()
                .unwrap()
                .add(Job::new(schedule.parse().unwrap(), move || {
                    robot.excute();
                }));
        }
    }
}
