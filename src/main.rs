#[macro_use]
extern crate rocket;
use std::{collections::HashMap};

use rocket::fs::{relative, FileServer};
use rocket::{tokio::sync::RwLock};
use rocket_dyn_templates::Template;
use guess_game::*;
mod api;
mod mcserver;
mod downloads;

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<&str, &str>::new())
}

// The same idea as using tokio::main I think
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/mcserver/", routes![mcserver::index])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api::poormaths::solve_eq, api::poormaths::solve_eq_error])
        .mount("/downloads", routes![downloads::downloads])
        .attach(Template::fairing())
        .manage(RwLock::new(Games::default()))
        .ignite()
        .await?;
    rocket.launch().await?;
    Ok(())
}
