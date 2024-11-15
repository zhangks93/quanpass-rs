
use salvo::prelude::*;
use crate::robot::{append_robot, get_robots, remove_robot, resume_robot, suspend_robot};

pub fn create_router() -> Router {
    Router::new().push(
        Router::with_path("robot")
            .get(get_robots)
            .post(append_robot)
            .delete(remove_robot)
            .push(Router::with_path("suspend").get(suspend_robot))
            .push(Router::with_path("resume").get(resume_robot))
    )

}