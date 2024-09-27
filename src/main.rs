use rocket::*;
// #[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "I'm running from Index!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, Rocket!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, hello])
}
