use std::{collections::HashMap, thread};

use salvo::{handler, Request, Response};

use crate::util::string_util::generate_random_id;

use self::robot::Robot;

pub mod robot;


#[handler]
pub fn get_robots(res: &mut Response) {
    res.render(salvo::prelude::Json(Robot::list()));
}

#[handler]
pub async fn append_robot(req: &mut Request, res: &mut Response)  -> Result<(), salvo::Error> {
    let params: HashMap<String, String> = match req.parse_form().await {
        Ok(data) => data,
        Err(_) => HashMap::new(),
    };
    let label = req.queries().get("label").cloned().unwrap_or_default();
    let name = req.queries().get("name").cloned().unwrap_or("Default Name".to_string());
    thread::spawn(|| {
        Robot::create(
            Robot::new(generate_random_id(), name, label, "0 1/2 * * * *".to_string(), params),
        );
    });
    res.render("add success");
    Ok(())
}

#[handler]
pub async fn remove_robot(req: &mut Request, res: &mut Response) {
    let id = req.queries().get("id").cloned().unwrap_or_default();
    thread::spawn(|| {
        Robot::remove(id);
    });
    res.render("remove success");
}

#[handler]
pub async fn suspend_robot(req: &mut Request, res: &mut Response) {
    let id = req.queries().get("id").cloned().unwrap_or_default();
    thread::spawn(|| {
        Robot::suspend(id);
    });
    res.render("suspend success");
}


#[handler]
pub async fn resume_robot(req: &mut Request, res: &mut Response) {
    let id = req.queries().get("id").cloned().unwrap_or_default();
    thread::spawn(|| {
        Robot::resume(id);
    });
    res.render("resume success");
}





