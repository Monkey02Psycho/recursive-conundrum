use std::collections::HashMap;

use rocket_dyn_templates::Template;

#[get("/")]
pub fn downloads() -> Template {
    Template::render("downloads", &HashMap::<&str, &str>::new())
}