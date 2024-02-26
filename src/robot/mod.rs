use std::{collections::HashMap, thread, time::Duration};

use salvo::{handler, Response};

use self::robot::Robot;

pub mod robot;

#[handler]
pub fn get_robots(res: &mut Response) {
    println!("{:?}", Robot::list());
    res.render("robots");
}

#[handler]
pub fn append_robot() -> &'static str {
    let mut params = HashMap::new();
    params.insert(String::from("quantity"), 10_f32);
    params.insert(String::from("gap"), 0.002_f32);
    let mut params_clone1 = params.clone();
    let mut params_clone2 = params.clone();
    thread::spawn(|| {
        Robot::append(
            Robot::new(
                String::from("ID"),
                String::from("Name"),
                String::from("Grid"),
                String::from("ARBFDUSD"),
                params,
            ),
            "0 1/5 * * * *",
        );
    });
    std::thread::sleep(Duration::from_millis(120000));
    params_clone1.insert(String::from("quantity"), 0.024_f32);
    thread::spawn(|| {
        Robot::append(
            Robot::new(
                String::from("ID"),
                String::from("Name"),
                String::from("Grid"),
                String::from("ORDIFDUSD"),
                params_clone1,
            ),
            "0 1/5 * * * *",
        );
    });
    std::thread::sleep(Duration::from_millis(120000));
    params_clone2.insert(String::from("quantity"), 20_f32);
    thread::spawn(|| {
        Robot::append(
            Robot::new(
                String::from("ID"),
                String::from("Name"),
                String::from("Grid"),
                String::from("SEIFDUSD"),
                params_clone2,
            ),
            "0 1/5 * * * *",
        );
    });
    return "Success";
}
