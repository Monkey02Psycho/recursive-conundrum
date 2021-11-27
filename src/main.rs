use std::sync::{RwLock, atomic::{AtomicU32, Ordering}};
#[macro_use]
extern crate rocket;
use rocket::{State, fs::{FileServer, relative}};

struct Count {
    count: AtomicU32,
}

// TODO turn into a request guard of sorts
// https://rocket.rs/v0.5-rc/guide/state/
struct Text {
    text: RwLock<String>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/count")]
fn count(hit_count: &State<Count>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}

#[get("/add")]
fn add(hit_count: &State<Count>) -> String {
    // let current = hit_count.count.load(Ordering::Relaxed);
    hit_count.count.fetch_add(1, Ordering::Relaxed);
    format!("Added 1")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, count, add, string, update])
        .manage(Count {
            count: AtomicU32::new(0),
        })
        .manage(Text {
            text: RwLock::new(String::new()),
        })
        .mount("/", FileServer::from(relative!("static")))
}
