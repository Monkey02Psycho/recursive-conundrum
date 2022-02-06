use std::{collections::HashMap, sync::RwLock};

use rocket::State;
use rocket_dyn_templates::{Template};
/// To add this to the site add these easy lines:
/// '''
/// .mount(
/// "/rwlock/",
/// routes![
///     rwlock::index,
///     rwlock::admin,
///     rwlock::user,
///     rwlock::update,
///     rwlock::string
/// ],
/// )
/// .manage(rwlock::Text::new())
/// '''
pub struct Text {
    pub text: RwLock<String>,
}

impl Text{
    pub fn new() -> Text{
        Text {
            text: RwLock::new(String::new())
        }
    }   
}

#[get("/")]
pub fn index() -> Template {
    Template::render("rwlock_index", &HashMap::<&str, &str>::new())
}

#[get("/string")]
pub fn string(text: &State<Text>) -> String {
    format!("{}", text.text.read().expect("Oh shit"))
}

#[get("/string/<new>")]
pub fn update(new: String, text: &State<Text>) -> String {
    let mut old = text.text.write().unwrap();
    *old = new;
    format!("String is: {}", &old)
}

#[get("/user")]
pub fn user() -> Template {
    Template::render("user", &HashMap::<&str, &str>::new())
}

#[get("/admin")]
pub fn admin() -> Template {
    Template::render("admin", &HashMap::<&str, &str>::new())
}