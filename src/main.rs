use rocket::*;
use std::{fs, io};
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello, delay, blocking_task])
        .launch()
        .await?;

    Ok(())
}
