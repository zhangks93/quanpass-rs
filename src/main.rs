mod client;
mod market;
mod notify;
mod robot;
mod strategy;
mod trade;
mod util;

use robot::{append_robot, get_robots, robot::MANAGER};
use salvo::prelude::*;
use std::{thread, time::Duration};

fn start_job_scheduler() {
    loop {
        unsafe {
            MANAGER.lock().unwrap().tick();
        }
        std::thread::sleep(Duration::from_millis(5000));
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("add_robot").get(append_robot))
        .push(Router::with_path("robot_list").get(get_robots));
    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    thread::spawn(|| {
        start_job_scheduler();
    });

    Server::new(acceptor).serve(router).await;
}
