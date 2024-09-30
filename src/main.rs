use diesel::prelude::*;
use rocket::{fs::FileServer, *};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocketing::routes::*;
use rocketing::{create_person, create_post, establish_connection, models::*};

#[database("my_pg_db")]
struct MyPgDatabase(PgConnection);

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

#[get("/new-post/<title>/<body>")]
async fn new_post(db: MyPgDatabase, title: &str, body: &str) {
    let title = title.to_string();
    let body = body.to_string();
    db.run(move |conn| create_post(conn, title, body)).await;
}

#[get("/create-person/<first>/<last>/<address>/<city>")]
async fn new_person(first: &str, last: &str, address: &str, city: &str) {
    let connection = &mut establish_connection();

    create_person(connection, first, last, address, city);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(MyPgDatabase::fairing())
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
        .register("/", catchers![internal_error, not_found, default])
        .launch()
        .await?;

    Ok(())
}
