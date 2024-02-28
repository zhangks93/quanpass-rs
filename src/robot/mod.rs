use std::{collections::HashMap, thread, time::Duration};

use salvo::{handler, Request, Response};

use self::robot::Robot;

pub mod robot;

#[handler]
pub fn get_robots(res: &mut Response) {
    println!("{:?}", Robot::list());
    res.render("robots");
}

#[handler]
pub async fn append_robot() -> &'static str {
    let mut params = HashMap::new();
    params.insert(String::from("quantity"), 12_f32);
    params.insert(String::from("gap"), 0.002_f32);
    let mut params_clone2 = params.clone();
    thread::spawn(|| {
        Robot::append(
            Robot::new(
                String::from("ID"),
                String::from("Name"),
                String::from("Grid"),
                String::from("PIXELFDUSD"),
                params,
            ),
            "0 1/4 * * * *",
        );
    });
    params_clone2.insert(String::from("quantity"), 2_f32);
    thread::spawn(|| {
        Robot::append(
            Robot::new(
                String::from("ID"),
                String::from("Name"),
                String::from("Grid"),
                String::from("MANTAFDUSD"),
                params_clone2,
            ),
            "0 1/4 * * * *",
        );
    });
    return "Success";
}

