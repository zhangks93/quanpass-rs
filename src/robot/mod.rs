use std::{collections::HashMap, thread};

use salvo::{handler, Request, Response};

use self::robot::Robot;

pub mod robot;

#[handler]
pub fn get_robots(res: &mut Response) {
    res.render(salvo::prelude::Json(Robot::list()));
}

#[handler]
pub async fn append_robot(req: &mut Request, res: &mut Response) {
    let params: HashMap<String, String> = match req.parse_form().await {
        Ok(data) => data,
        Err(_) => HashMap::new(),
    };
    let label = req.queries().get("label").cloned().unwrap_or_default();
    thread::spawn(|| {
        Robot::append(
            Robot::new(String::from("ID"), String::from("Name"), label, params),
            "0 1/3 * * * *",
        );
    });
    res.render("add success");
}

#[handler]
pub async fn remove_robot(req: &mut Request, res: &mut Response) {
    let id = req.queries().get("id").cloned().unwrap_or_default();
    thread::spawn(|| {
        Robot::remove(id);
    });
    res.render("remove success");
}
