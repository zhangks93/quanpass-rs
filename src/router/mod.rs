pub mod robot_router;

use salvo::prelude::*;

pub fn create_router() -> Router {
    let robot_router = robot_router::create_router();

    Router::new().push(robot_router)
}
