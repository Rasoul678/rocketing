use crate::{add_todo, establish_connection, models::*};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::*;
use rocket_dyn_templates::{context, Template};

use super::MyPgDatabase;

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Home",
        },
    )
}

#[get("/todos")]
pub async fn todos() -> Template {
    use crate::schema::todos::dsl::*;
    use rocket::serde::Serialize;

    let connection = &mut establish_connection();

    let all_todos = todos
        .limit(15)
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading todos");

    #[derive(Serialize)]
    struct SerializableTodo {
        id: i32,
        title: String,
        body: String,
        comopleted: bool,
        created_at: String,
    }

    let serializable_todos: Vec<SerializableTodo> = all_todos
        .into_iter()
        .map(|todo| {
            let c_at: chrono::DateTime<chrono::Utc> = todo.created_at.into();

            SerializableTodo {
                id: todo.id,
                title: todo.title,
                body: todo.body,
                comopleted: todo.completed,
                created_at: c_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            }
        })
        .collect();

    Template::render(
        "tera/todos",
        context! {
            title: "Todos",
            todos: serializable_todos,
        },
    )
}

#[get("/todos/new")]
pub async fn new_todo() -> Template {
    Template::render(
        "tera/edit-todo",
        context! {
            title: "Create",
            name: "",
            body: ""
        },
    )
}

#[derive(FromForm, Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    body: String,
}

#[post("/todos/create", data = "<todo>")]
pub async fn create_todo(db: MyPgDatabase, todo: Form<CreateTodo>) -> Redirect {
    let title = todo.title.clone();
    let body = todo.body.clone();

    let new_todo = db.run(move |conn| add_todo(conn, title, body)).await;

    match new_todo {
        Ok(_) => Redirect::to("/todos"),
        // TODO: replace with nice html error page
        Err(_) => panic!("Error saving new todo"),
    }
}

#[get("/todos/edit/<t_id>")]
pub async fn update_todo(db: MyPgDatabase, t_id: &str) -> Template {
    use crate::schema::todos::dsl::*;

    let todo_id = t_id.parse::<i32>().unwrap_or_default();

    let todo = db
        .run(move |conn| {
            todos
                .filter(id.eq(todo_id))
                .select(Todo::as_select())
                .first(conn)
                .optional()
                .expect("Error loading posts")
        })
        .await;

    match todo {
        Some(todo) => Template::render(
            "tera/edit-todo",
            context! {
                title: "Update",
                name: todo.title,
                body: todo.body
            },
        ),
        None => Template::render(
            "tera/error/404",
            context! {
                title: "Update",
                uri: format!("/todo/{t_id}")
            },
        ),
    }
}
