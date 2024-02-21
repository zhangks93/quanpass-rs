use std::{collections::HashMap, thread};

use salvo::{handler, Response, };

use self::robot::Robot;

pub mod robot;

#[handler]
pub fn get_robots(res: &mut Response) {
    println!("{:?}", Robot::active_list());
    res.render("robots");
}

#[handler]
pub fn append_robot() -> &'static str {
    let mut params = HashMap::new();
    params.insert(String::from("quantity"), 2_f32);
    params.insert(String::from("gap"), 0.003_f32);
    thread::spawn(|| {
    Robot::append(
        Robot::new(
            String::from("ID"),
            String::from("Name"),
            String::from("Grid"),
            String::from("STRKFDUSD"),
            params,
        ),
        "0 1/2 * * * *",
    );});
    return "Success";
}
