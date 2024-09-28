use rocket::{
    fs::{FileServer, NamedFile},
    *,
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tokio::time::{sleep, Duration};
// #[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocketing::{create_post, establish_connection, models::*, schema::posts::title};

#[get("/")]
fn index() -> &'static str {
    "I'm running from Index!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, Rocket!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec =
        rocket::tokio::task::spawn_blocking(|| fs::read(Path::new("www/static/").join("data.txt")))
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/page/<path..>")]
fn get_page(path: PathBuf) {
    for p in &path {
        println!("{:#?}", p);
    }

    println!("{}", path.display());
}

#[get("/public/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/static/").join(file))
        .await
        .ok()
}

#[get("/users")]
async fn users() {
    use rocketing::schema::persons::dsl::*;

    let connection = &mut establish_connection();

    let db_persons: Vec<Person> = persons
        .limit(5)
        .select(Person::as_select())
        .load(connection)
        .expect("Error loading persons");

    println!("Displaying {} persons", db_persons.len());

    for person in db_persons {
        println!("Firstname: {}", person.firstname.unwrap());
        println!("-----------");
        println!("Lastname: {}", person.lastname.unwrap());
    }
}

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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                index,
                hello,
                delay,
                blocking_task,
                get_page,
                users,
                posts,
                new_post
            ],
        )
        .mount("/public", FileServer::from("www/static/"))
        .launch()
        .await?;

    Ok(())
}
