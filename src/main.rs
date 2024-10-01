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
                users,
                todos,
                new_post,
                new_person,
                update_todo,
                new_todo,
                create_todo
            ],
        )
        .mount("/", FileServer::from("www/static/"))
        .register("/", catchers![internal_error, not_found, default])
        .launch()
        .await?;

    Ok(())
}
