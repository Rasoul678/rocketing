pub mod models;
pub mod routes;
pub mod schema;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use models::{NewPerson, NewPost, NewTodo, Person, Post, Todo};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, ref title: String, ref body: String) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn add_todo(
    conn: &mut PgConnection,
    ref title: String,
    ref body: String,
) -> Result<Todo, DieselError> {
    use crate::schema::todos;

    let new_todo = NewTodo { title, body };

    let todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)?;

    Ok(todo)
}

pub fn create_person(
    conn: &mut PgConnection,
    firstname: &str,
    lastname: &str,
    address: &str,
    city: &str,
) -> Person {
    use crate::schema::persons;

    let new_person = NewPerson {
        firstname: Some(firstname),
        lastname: Some(lastname),
        address: Some(address),
        city: Some(city),
    };

    diesel::insert_into(persons::table)
        .values(&new_person)
        .returning(Person::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
