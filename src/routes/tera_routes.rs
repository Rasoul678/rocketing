use crate::{
    add_todo, add_user, get_user_by_email, get_user_todos, hash_password, models::*, update_todo,
    AuthUser, LoginUserForm, RegisterUserForm, TodoForm,
};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket::form::Form;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::json::to_string;
use rocket::*;
use rocket_dyn_templates::{context, Template};
use std::path::PathBuf;

use super::MyPgDatabase;

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![default, internal_error, not_found]
}

#[get("/")]
fn index(flash: Option<FlashMessage<'_>>, user: AuthUser) -> Template {
    let msg_type = flash
        .as_ref()
        .map(|flash| flash.kind().to_string())
        .unwrap_or_default();

    let msg_content = flash
        .map(|flash| format!("{}", flash.message()))
        .unwrap_or_default();

    Template::render(
        "tera/index",
        context! {
            title: "Home",
            msg_type,
            msg_content,
            user
        },
    )
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!(login_view))
}

#[get("/todos")]
async fn todos(db: MyPgDatabase, flash: Option<FlashMessage<'_>>, user: AuthUser) -> Template {
    use rocket::serde::Serialize;

    let msg_type = flash
        .as_ref()
        .map(|flash| flash.kind().to_string())
        .unwrap_or_default();

    let msg_content = flash
        .map(|flash| format!("{}", flash.message()))
        .unwrap_or_default();

    let all_todos = db
        .run(move |conn| get_user_todos(conn, user.id))
        .await
        .unwrap();

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
            msg_content,
            user,
        },
    )
}

#[get("/todos", rank = 2)]
fn no_auth_todos() -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!(login)), "Sorry, you must login first")
}

#[get("/todos/new")]
async fn create_todo_view(flash: Option<FlashMessage<'_>>, user: AuthUser) -> Template {
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
            msg: flash_msg,
            user,
        },
    )
}

#[get("/todos/new", rank = 2)]
fn no_auth_create_todo_view() -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!(index)), "Sorry, you must login first")
}

#[post("/todos/create", data = "<todo>")]
async fn create_todo_action(
    db: MyPgDatabase,
    todo: Form<TodoForm>,
    user: AuthUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let title = todo.title.trim().to_string();
    let body = todo.body.trim().to_string();

    if title.is_empty() || body.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(create_todo_view)),
            "Title and Text are required!",
        ));
    }

    let new_todo = db
        .run(move |conn| add_todo(conn, title, body, user.id))
        .await;

    match new_todo {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(todos)),
            "New Todo successfully added",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(create_todo_view)),
            "Error saving new todo",
        )),
    }
}

#[get("/todos/edit/<t_id>")]
async fn update_todo_view(
    db: MyPgDatabase,
    t_id: &str,
    flash: Option<FlashMessage<'_>>,
    user: AuthUser,
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
                msg: flash_msg,
                user
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

#[get("/todos/edit/<_id..>", rank = 2)]
fn no_auth_update_todo_view(_id: PathBuf) -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!(index)), "Sorry, you must login first")
}

#[post("/todos/update", data = "<todo>")]
async fn update_todo_action(
    db: MyPgDatabase,
    todo: Form<TodoForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let title = todo.title.trim().to_string();
    let body = todo.body.trim().to_string();
    let todo_id = todo.id.unwrap_or_default();

    if title.is_empty() || body.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(update_todo_view(todo_id.to_string()))),
            "Title and Text are required!",
        ));
    }

    let new_todo = db
        .run(move |conn| update_todo(conn, todo_id, title, body))
        .await;

    match new_todo {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(todos)),
            "Todo successfully updated",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(todos)),
            "Error updating new todo",
        )),
    }
}

#[get("/auth/register")]
fn register(_user: AuthUser) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/auth/register", rank = 2)]
async fn register_view(flash: Option<FlashMessage<'_>>) -> Template {
    let flash_msg = flash
        .map(|flash| format!("{}: {}", flash.kind(), flash.message()))
        .unwrap_or_default();

    Template::render(
        "tera/form-user",
        context! {
            title: "REGISTER",
            msg: flash_msg
        },
    )
}

#[post("/auth/register", data = "<user>")]
async fn register_action(
    db: MyPgDatabase,
    user: Form<RegisterUserForm>,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let name = user.name.to_lowercase().trim().to_string();
    let email = user.email.to_lowercase().trim().to_string();
    let password = user.password.trim().to_string();

    if name.is_empty() || password.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(register_view)),
            "Name and Password are required!",
        ));
    }

    let password_hash = hash_password(&password);

    let new_user = db
        .run(move |conn| add_user(conn, name, email, password_hash))
        .await;

    match new_user {
        Ok(user) => {
            let cookie = AuthUser {
                id: user.id,
                name: user.name,
                email: user.email,
            };

            cookies.add_private(Cookie::new("user", to_string(&cookie).unwrap()));

            Ok(Flash::success(
                Redirect::to(uri!(index)),
                "New User successfully logged in",
            ))
        }
        Err(err) => match err {
            DieselError::QueryBuilderError(msg) => Err(Flash::error(
                Redirect::to(uri!(register_view)),
                msg.to_string(),
            )),
            _ => Err(Flash::error(
                Redirect::to(uri!(register_view)),
                "Error registering new user",
            )),
        },
    }
}

#[get("/auth/login")]
fn login(_user: AuthUser) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/auth/login", rank = 2)]
fn login_view(flash: Option<FlashMessage<'_>>) -> Template {
    let flash_msg = flash
        .map(|flash| format!("{}: {}", flash.kind(), flash.message()))
        .unwrap_or_default();

    Template::render(
        "tera/form-user",
        context! {
            title: "LOGIN",
            msg: flash_msg
        },
    )
}

#[post("/auth/login", data = "<user>")]
async fn login_action(
    db: MyPgDatabase,
    user: Form<LoginUserForm>,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let email = user.email.to_lowercase().trim().to_string();
    let password = user.password.trim().to_string();

    if email.is_empty() || password.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(login_view)),
            "Email and Password are required!",
        ));
    }

    let password_hash = hash_password(&password);

    let user_result = db.run(move |conn| get_user_by_email(conn, &email)).await;

    match user_result {
        Ok(result) => match result {
            Some(user) => {
                if user.password_hash != password_hash {
                    return Err(Flash::error(
                        Redirect::to(uri!(login_view)),
                        "Invalid email or password",
                    ));
                }

                let cookie = AuthUser {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                };

                cookies.add_private(Cookie::new("user", to_string(&cookie).unwrap()));

                Ok(Flash::success(
                    Redirect::to(uri!(index)),
                    "New User successfully logged in",
                ))
            }
            None => {
                return Err(Flash::error(
                    Redirect::to(uri!(login_view)),
                    "Invalid email or password",
                ));
            }
        },
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(login_view)),
            "Error logging user in",
        )),
    }
}

#[post("/auth/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private("user");
    Flash::success(Redirect::to(uri!(index)), "User successfully logged out")
}

pub fn tera_routes() -> Vec<rocket::Route> {
    routes![
        index,
        no_auth_index,
        todos,
        no_auth_todos,
        create_todo_view,
        no_auth_create_todo_view,
        create_todo_action,
        update_todo_view,
        no_auth_update_todo_view,
        update_todo_action,
        register,
        register_view,
        register_action,
        login,
        login_view,
        login_action,
        logout
    ]
}
