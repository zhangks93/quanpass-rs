use crate::strategy::{
    self,
    strategy::{Strategy, StrategyFactory},
};
use job_scheduler::{Job, JobScheduler, Uuid};
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use std::{borrow::Borrow, collections::HashMap, sync::Mutex};

pub static mut MANAGER: Lazy<Mutex<JobScheduler>> = Lazy::new(|| Mutex::new(JobScheduler::new()));
pub static mut ACTIVE_ROBOTS: Lazy<Mutex<HashMap<Uuid, Robot>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Running,
    Suspended,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Running => "running".to_string(),
            Status::Suspended => "suspended".to_string(),
        }
    }
}

pub struct Robot {
    id: String,
    name: String,
    strategy: Box<dyn Strategy>,
    status: Status,
    schedule: String,
}

impl Clone for Robot {
    fn clone(&self) -> Self {
        Robot {
            id: self.id.clone(),
            name: self.name.clone(),
            strategy: self.strategy.clone_box(),
            status: self.status.clone(),
            schedule: self.schedule.clone(),
        }
    }
}

impl Robot {
    pub fn new(
        id: String,
        name: String,
        strategy: String,
        schedule: String,
        params: HashMap<String, String>,
    ) -> Robot {
        Robot {
            id: id,
            name: name,
            strategy: StrategyFactory::new().create_strategy(&strategy, params),
            schedule: schedule,
            status: Status::Running,
        }
    }

    pub fn to_map(&self) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("id".to_string(), Value::String(self.id.clone()));
        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert(
            "strategy".to_string(),
            Value::Object(self.strategy.to_json()),
        );
        map.insert("status".to_string(), Value::String(self.status.to_string()));
        return map;
    }

    pub fn create(robot: Robot) {
        let uuid = Robot::add_task(robot.clone());
        Robot::append(robot, uuid);
    }

    pub fn add_task(robot: Robot) -> Uuid {
        unsafe {
            let uuid =
                MANAGER
                    .lock()
                    .unwrap()
                    .add(Job::new(robot.schedule.parse().unwrap(), move || {
                        robot.excute();
                    }));
            return uuid;
        }
    }

    pub fn append(robot: Robot, uuid: Uuid) -> String {
        unsafe {
            ACTIVE_ROBOTS.lock().unwrap().insert(
                uuid,
                Robot::new(
                    uuid.to_string(),
                    robot.name,
                    robot.strategy.name(),
                    robot.schedule,
                    robot.strategy.params(),
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
            if is_removed {
                ACTIVE_ROBOTS
                    .lock()
                    .unwrap()
                    .remove(&Uuid::parse_str(uuid.as_str()).unwrap());
                return true;
            }
            return false;
        }
    }

    pub fn suspend(uuid: String) -> bool {
        unsafe {
            let is_suspended = MANAGER
                .lock()
                .unwrap()
                .remove(Uuid::parse_str(uuid.as_str()).unwrap());
            if is_suspended {
                let mut robots = ACTIVE_ROBOTS.lock().unwrap();
                let robot = robots
                    .get_mut(&Uuid::parse_str(uuid.as_str()).unwrap())
                    .unwrap();
                robot.status = Status::Suspended;
                return true;
            }
            return false;
        }
    }

    pub fn resume(uuid: String) -> bool {
        unsafe {
            let removed = ACTIVE_ROBOTS
                .lock()
                .unwrap()
                .remove(&Uuid::parse_str(uuid.as_str()).unwrap());
            if removed.is_some() {
                let robot = removed.unwrap();
                Robot::create(robot);
                return true;
            }
            return false;
        }
    }

    pub fn list() -> Vec<Map<String, Value>> {
        unsafe {
            return ACTIVE_ROBOTS
                .lock()
                .unwrap()
                .values()
                .map(|robot| robot.to_map())
                .collect();
        }
    }

    pub fn excute(&self) {
        self.strategy.excute();
    }
}
