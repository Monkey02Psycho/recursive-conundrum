use std::collections::HashMap;

use rocket_dyn_templates::{Template};

#[get("/")]
pub fn index() -> Template{
    Template::render("mcserver/index", &HashMap::<&str, &str>::new())
}