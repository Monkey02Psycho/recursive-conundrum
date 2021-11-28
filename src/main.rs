use std::{collections::HashMap, sync::{RwLock, atomic::{AtomicU32, Ordering}}};
#[macro_use]
extern crate rocket;
use rocket::{State, fs::{FileServer, relative}};

use rocket_dyn_templates::{Template};

// TODO turn into a request guard of sorts
// https://rocket.rs/v0.5-rc/guide/state/
struct Text {
    text: RwLock<String>,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<&str, &str>::new())
}

#[get("/string")]
fn string(text: &State<Text>) -> String {
    format!("{}", text.text.read().expect("Oh shit"))
}

#[get("/string/<new>")]
fn update(new: String, text: &State<Text>) -> String {
    let mut old = text.text.write().unwrap();
    *old = new;
    format!("String is: {}", &old)
}

#[get("/user")]
fn user() -> Template {
    Template::render("user", &HashMap::<&str, &str>::new())
}

#[get("/admin")]
fn admin() -> Template {
    Template::render("admin", &HashMap::<&str, &str>::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, string, update, admin, user])
        .mount("/", FileServer::from(relative!("static")))
        .manage(Text {
            text: RwLock::new(String::new()),
        }).attach(Template::fairing())
}
