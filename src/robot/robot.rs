use crate::strategy::strategy::{Strategy, StrategyFactory};
use job_scheduler::{Job, JobScheduler, Uuid};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static mut MANAGER: Lazy<Mutex<JobScheduler>> = Lazy::new(|| Mutex::new(JobScheduler::new()));
pub static mut ACTIVE_ROBOTS: Lazy<Mutex<HashMap<Uuid, Robot>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Robot {
    id: String,
    name: String,
    strategy: Box<dyn Strategy>,
}

impl Clone for Robot {
    fn clone(&self) -> Self {
        Robot {
            id: self.id.clone(),
            name: self.name.clone(),
            strategy: self.strategy.clone_box(), // Assume clone_box is implemented for Strategy
        }
    }
}

impl Robot {
    pub fn new(
        id: String,
        name: String,
        strategy: String,
        symbol: String,
        params: HashMap<String, f32>,
    ) -> Robot {
        Robot {
            id: id,
            name: name,
            strategy: StrategyFactory::new().create_strategy(&strategy, symbol, params),
        }
    }

    pub fn excute(&self) {
        self.strategy.excute();
    }

    pub fn append(robot: Robot, schedule: &str) {
        unsafe {
            
            let uuid =
                MANAGER
                    .lock()
                    .unwrap()
                    .add(Job::new(schedule.parse().unwrap(), move || {
                        robot.excute();
                    }));
            ACTIVE_ROBOTS.lock().unwrap().insert(
                uuid,
                Robot::new(
                    uuid.to_string(),
                    String::from("name"),
                    String::from("strategy"),
                    String::from("strategy"),
                    HashMap::new(),
                ),
            );
        }
    }

    pub fn active_list() -> Vec<String> {
        unsafe {
            return ACTIVE_ROBOTS
                .lock()
                .unwrap()
                .values()
                .map(|robot| robot.id.clone())
                .collect();
        }
    }
}
