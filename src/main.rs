#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::{Template};

mod rwlock;

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<&str, &str>::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/rwlock/", routes![rwlock::index, rwlock::admin, rwlock::user, rwlock::update, rwlock::string])
        .mount("/", FileServer::from(relative!("static")))
        .manage(rwlock::Text::new())
        .attach(Template::fairing())
}
