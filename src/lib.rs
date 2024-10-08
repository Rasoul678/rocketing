pub mod models;
pub mod routes;
pub mod schema;
pub mod types;

use crypto::{digest::Digest, sha3::Sha3};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use models::{NewTodo, NewUser, Todo, User};
use std::env;
use types::*;

pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

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
    user_id: i32,
) -> Result<Todo, DieselError> {
    use crate::schema::todos;

    let new_todo = NewTodo {
        title,
        body,
        user_id,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
}

pub fn update_todo(
    conn: &mut PgConnection,
    todo_id: i32,
    ref new_title: String,
    ref new_body: String,
) -> Result<Todo, DieselError> {
    use crate::schema::todos::dsl::*;

    diesel::update(todos.find(todo_id))
        .set((title.eq(new_title), body.eq(new_body)))
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
        .set(completed.eq(diesel::dsl::not(completed)))
        .returning(Todo::as_returning())
        .get_result(conn)
}

pub fn add_user(
    conn: &mut PgConnection,
    ref name: String,
    ref email: String,
    ref password_hash: String,
) -> Result<User, DieselError> {
    use crate::schema::users;

    let exists = get_user_by_email(conn, &email).unwrap_or_default();

    if exists.is_some() {
        return Err(DieselError::QueryBuilderError(
            "User with this email already exists".into(),
        ));
    }

    let new_user = NewUser {
        name,
        email,
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn get_user_by_email(
    conn: &mut PgConnection,
    user_email: &str,
) -> Result<Option<User>, DieselError> {
    use crate::schema::users::dsl::*;
    users
        .filter(email.eq(user_email))
        .select(User::as_select())
        .first(conn)
        .optional()
}

pub fn get_user_todos(conn: &mut PgConnection, uid: i32) -> Result<Vec<Todo>, DieselError> {
    use crate::schema::todos::dsl::*;

    todos
        .filter(user_id.eq(uid))
        .limit(15)
        .order_by(created_at.desc())
        .select(Todo::as_select())
        .load(conn)
}
