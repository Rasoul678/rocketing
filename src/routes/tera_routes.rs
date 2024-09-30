use rocket::http::Status;
use rocket::*;
use rocket_dyn_templates::{context, Template};

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Home",
        },
    )
}
