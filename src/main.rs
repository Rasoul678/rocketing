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
    let vec = rocket::tokio::task::spawn_blocking(|| fs::read("data.txt"))
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello, delay, blocking_task, get_page])
        .mount("/public", FileServer::from("www/static/").rank(-20))
        .launch()
        .await?;

    Ok(())
}
