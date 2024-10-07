use crate::{add_todo, establish_connection, models::*, update_todo};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
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
pub async fn todos(flash: Option<FlashMessage<'_>>) -> Template {
    use crate::schema::todos::dsl::*;
    use rocket::serde::Serialize;

    let msg_type = flash
        .as_ref()
        .map(|flash| flash.kind().to_uppercase())
        .unwrap_or_default();

    let msg_content = flash
        .map(|flash| format!("{}", flash.message()))
        .unwrap_or_default();

    let connection = &mut establish_connection();

    let all_todos = todos
        .limit(15)
        .select(Todo::as_select())
        .order_by(created_at.desc())
        .load(connection)
        .expect("Error loading todos");

    #[derive(Serialize)]
    struct SerializableTodo {
        id: i32,
        title: String,
        body: String,
        completed: bool,
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
                completed: todo.completed,
                created_at: c_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            }
        })
        .collect();

    Template::render(
        "tera/todos",
        context! {
            title: "Todos",
            todos: serializable_todos,
            msg_type,
            msg_content
        },
    )
}

#[get("/todos/new")]
pub async fn create_view(flash: Option<FlashMessage<'_>>) -> Template {
    let flash_msg = flash
        .map(|flash| format!("{}: {}", flash.kind(), flash.message()))
        .unwrap_or_default();

    Template::render(
        "tera/form-todo",
        context! {
            title: "Create",
            name: "",
            body: "",
            id: 0,
            msg: flash_msg
        },
    )
}

#[derive(FromForm, Debug, Deserialize)]
pub struct TodoForm {
    title: String,
    body: String,
    id: Option<i32>,
}

#[post("/todos/create", data = "<todo>")]
pub async fn create_action(
    db: MyPgDatabase,
    todo: Form<TodoForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let title = todo.title.trim().to_string();
    let body = todo.body.trim().to_string();

    if title.is_empty() || body.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(create_view)),
            "Title and Text are required!",
        ));
    }

    let new_todo = db.run(move |conn| add_todo(conn, title, body)).await;

    match new_todo {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!("/todos")),
            "New Todo successfully added",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(todos)),
            "Error saving new todo",
        )),
    }
}
#[get("/todos/edit/<t_id>")]
pub async fn update_view(
    db: MyPgDatabase,
    t_id: &str,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    use crate::schema::todos::dsl::*;

    let flash_msg = flash
        .map(|flash| format!("{}: {}", flash.kind(), flash.message()))
        .unwrap_or_default();

    let todo_id = t_id.parse::<i32>().unwrap_or_default();

    let todo = db
        .run(move |conn| {
            todos
                .filter(id.eq(todo_id))
                .filter(completed.eq(false))
                .select(Todo::as_select())
                .first(conn)
                .optional()
                .expect("Error loading todo")
        })
        .await;

    match todo {
        Some(todo) => Template::render(
            "tera/form-todo",
            context! {
                title: "Update",
                name: todo.title,
                body: todo.body,
                id: todo_id,
                msg: flash_msg
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

#[post("/todos/update", data = "<todo>")]
pub async fn update_action(
    db: MyPgDatabase,
    todo: Form<TodoForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let title = todo.title.trim().to_string();
    let body = todo.body.trim().to_string();
    let todo_id = todo.id.unwrap_or_default();

    if title.is_empty() || body.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(update_view(todo_id.to_string()))),
            "Title and Text are required!",
        ));
    }

    let new_todo = db
        .run(move |conn| update_todo(conn, todo_id, title, body))
        .await;

    match new_todo {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!("/todos")),
            "Todo successfully updated",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(todos)),
            "Error updating new todo",
        )),
    }
}
