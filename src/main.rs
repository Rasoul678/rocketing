use rocket::{fs::FileServer, *};
// #[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocketing::{create_person, create_post, establish_connection, models::*};

use rocketing::routes::*;

use rocket_dyn_templates::Template;
// extern crate rocket_dyn_templates;

#[get("/posts")]
async fn posts() {
    use rocketing::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("Title: {}", post.title);
        println!("-----------");
        println!("Body: {}", post.body);
    }
}

#[get("/create-post/<_title>/<body>")]
async fn new_post(_title: &str, body: &str) {
    let connection = &mut establish_connection();

    create_post(connection, _title, body);
}

#[get("/create-person/<first>/<last>/<address>/<city>")]
async fn new_person(first: &str, last: &str, address: &str, city: &str) {
    let connection = &mut establish_connection();

    create_person(connection, first, last, address, city);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                simple_hello,
                tera_hello,
                delay,
                blocking_task,
                get_page,
                users,
                posts,
                new_post,
                new_person
            ],
        )
        .mount("/public", FileServer::from("www/static/"))
        .register("/*", catchers![not_found])
        .launch()
        .await?;

    Ok(())
}
