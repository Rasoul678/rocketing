use rocket::{fs::FileServer, *};
use rocket_dyn_templates::Template;
use rocketing::routes::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(MyPgDatabase::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                todos,
                create_todo_view,
                create_todo_action,
                update_todo_view,
                update_todo_action,
            ],
        )
        .mount("/api", routes![remove, complete])
        .mount("/", FileServer::from("www/static/"))
        .register("/", catchers![internal_error, not_found, default])
        .launch()
        .await?;

    Ok(())
}
