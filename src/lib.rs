pub mod models;
pub mod routes;
pub mod schema;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use models::{NewTodo, Todo};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_todo(
    conn: &mut PgConnection,
    ref title: String,
    ref body: String,
) -> Result<Todo, DieselError> {
    use crate::schema::todos;

    let new_todo = NewTodo { title, body };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
}

pub fn delete_todo(conn: &mut PgConnection, todo_id: i32) -> Result<usize, DieselError> {
    use crate::schema::todos::dsl::*;
    diesel::delete(todos.find(todo_id)).execute(conn)
}

pub fn complete_todo(conn: &mut PgConnection, todo_id: i32) -> Result<Todo, DieselError> {
    use crate::schema::todos::dsl::*;

    diesel::update(todos.find(todo_id))
        .set(completed.eq(true))
        .returning(Todo::as_returning())
        .get_result(conn)
}
