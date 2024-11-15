mod client;
mod market;
mod notify;
mod robot;
mod strategy;
mod crypto;
mod util;
mod router;

use robot::{append_robot, get_robots, remove_robot, robot::MANAGER};
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
    thread::spawn(|| {
        start_job_scheduler();
    });

    let router = router::create_router(); 
    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;
    let service = Service::new(router);

    Server::new(acceptor).serve(service).await;
}
