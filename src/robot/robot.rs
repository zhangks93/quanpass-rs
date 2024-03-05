use crate::strategy::strategy::{Strategy, StrategyFactory};
use job_scheduler::{Job, JobScheduler, Uuid};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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
            strategy: self.strategy.clone_box(),
        }
    }
}

impl Robot {
    pub fn new(
        id: String,
        name: String,
        strategy: String,
        params: HashMap<String, String>,
    ) -> Robot {
        Robot {
            id: id,
            name: name,
            strategy: StrategyFactory::new().create_strategy(&strategy, params),
        }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("id".to_string(), self.id.clone());
        map.insert("name".to_string(), self.name.clone());
        return map;
    }

    pub fn excute(&self) {
        self.strategy.excute();
    }

    pub fn append(robot: Robot, schedule: &str) -> String {
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
                    HashMap::new(),
                ),
            );
            return uuid.to_string();
        }
    }

    pub fn remove(uuid: String) -> bool {
        unsafe {
            let is_removed = MANAGER
                .lock()
                .unwrap()
                .remove(Uuid::parse_str(uuid.as_str()).unwrap());
            ACTIVE_ROBOTS
                .lock()
                .unwrap()
                .remove(&Uuid::parse_str(uuid.as_str()).unwrap());
            return is_removed;
        }
    }

    pub fn list() -> Vec<HashMap<String, String>> {
        unsafe {
            return ACTIVE_ROBOTS
                .lock()
                .unwrap()
                .values()
                .map(|robot| robot.to_map())
                .collect();
        }
    }
}
