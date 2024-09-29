use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket::tokio::time::*;

use std::path::{Path, PathBuf};
use std::{fs, io};

mod db_routes;
mod tera_routes;

pub use db_routes::users;
pub use tera_routes::{not_found, tera_hello};

#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(r#"<h1>I'm an Raw HTML running from Index!</h1>"#)
}

#[get("/hello")]
pub fn simple_hello() -> &'static str {
    "Hello, Rocket!"
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec =
        rocket::tokio::task::spawn_blocking(|| fs::read(Path::new("www/static/").join("data.txt")))
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/page/<path..>")]
pub fn get_page(path: PathBuf) {
    for p in &path {
        println!("{:#?}", p);
    }

    println!("{}", path.display());
}

#[get("/public/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/static/").join(file))
        .await
        .ok()
}
