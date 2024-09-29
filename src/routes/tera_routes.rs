use rocket::*;
use rocket_dyn_templates::{context, Template};

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[get("/hello/<name>")]
pub fn tera_hello(name: &str) -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}
