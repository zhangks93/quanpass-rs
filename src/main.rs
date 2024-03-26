mod client;
mod market;
mod notify;
mod robot;
mod strategy;
mod crypto;
mod util;

use robot::{append_robot, get_robots, remove_robot, robot::MANAGER};
use salvo::{cors::Cors, http::Method};
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

    let router = Router::new().push(
        Router::with_path("robot")
            .get(get_robots)
            .post(append_robot)
            .delete(remove_robot),
    );
    let cors = Cors::new()
        .allow_origin("http://158.247.243.188")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();
    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;
    let service = Service::new(router).hoop(cors);

    Server::new(acceptor).serve(service).await;
}
