use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocketing::routes::{api_routes, catchers, tera_routes, *};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(MyPgDatabase::fairing())
        .attach(Template::fairing())
        .mount("/", tera_routes())
        .mount("/api", api_routes())
        .mount("/", FileServer::from("www/static/"))
        .register("/", catchers())
        .launch()
        .await?;

    Ok(())
}
