mod market;
mod notify;
mod robot;
mod strategy;
mod util;
mod trade;
mod client;

use std::collections::HashMap;
use std::time::Duration;

use robot::robot::Robot;
use robot::robot::MANAGER;

#[tokio::main]
async fn main() {

    let mut params = HashMap::new();
    params.insert(String::from("quantity"), 150.0_f32);
    params.insert(String::from("gap"), 0.0007_f32);
    Robot::append(Robot::new(String::from("Jack"), String::from("Grid"), String::from("DOGEBUSD"), params), "0 1/2 * * * *");
    
    // trade::crypto_client::CryptoClient::new().limit_buy("DOGEBUSD", 150.0, 0.08158);

    loop {
        unsafe {
            MANAGER.lock().unwrap().tick();
            println!("{:?}", MANAGER.lock().unwrap().time_till_next_job());
            
            
            
        }
        std::thread::sleep(Duration::from_millis(5000));
        println!("Sleep 5 Seconds!");
        
    }
}
