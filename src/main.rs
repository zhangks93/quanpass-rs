mod market;
mod notify;
mod robot;
mod strategy;
mod util;
mod trade;

use std::time::Duration;

use robot::robot::Robot;
use robot::robot::MANAGER;

#[tokio::main]
async fn main() {
    Robot::append(Robot::new(String::from("Jack"), String::from("Grid")));
    trade::crypto_client::CryptoClient::new().limit_buy("DOGEBUSD", 150, 0.08158);

    loop {
        unsafe {
            MANAGER.lock().unwrap().tick();
        }
        std::thread::sleep(Duration::from_millis(5000));
    }
}
