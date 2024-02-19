use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    /* print!("begin");
    let mut params = HashMap::new();
    params.insert(String::from("quantity"), 10_f32);
    params.insert(String::from("gap"), 0.003_f32);
    //Robot::append(Robot::new(String::from("Jack"), String::from("Grid"), String::from("SXPBUSD"), params), "0 1/2 * * * *");
    Robot::append(Robot::new(String::from("Rose"), String::from("Triangle"), String::from("FILFDUSD;ICPFDUSD"), params), "0 1/2 * * * *");
    trade::crypto_client::CryptoClient::new().limit_buy("DOGEFDUSD", 100.0, 0.08158);


    loop {
        unsafe {
            MANAGER.lock().unwrap().tick();
        }
        std::thread::sleep(Duration::from_millis(5000));

    } */
    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    Server::new(acceptor).serve(router).await;
}
