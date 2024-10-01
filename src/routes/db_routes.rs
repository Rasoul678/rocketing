use crate::{add_todo, create_person, create_post, establish_connection, models::*};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::*;
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::database;

#[database("my_pg_db")]
pub struct MyPgDatabase(PgConnection);

#[get("/users")]
pub async fn users() -> Template {
    use crate::schema::persons::dsl::*;
    use rocket::serde::Serialize;

    let connection = &mut establish_connection();

    let people: Vec<Person> = persons
        .limit(5)
        .select(Person::as_select())
        .load(connection)
        .expect("Error loading persons");

    #[derive(Serialize)]
    struct SerializablePerson {
        firstname: Option<String>,
        lastname: Option<String>,
        city: Option<String>,
    }

    let serializable_people: Vec<SerializablePerson> = people
        .into_iter()
        .map(|person| SerializablePerson {
            firstname: person.firstname,
            lastname: person.lastname,
            city: person.city,
        })
        .collect();

    Template::render(
        "tera/users",
        context! {
            title: "Users",
            name: Some("Rasoul"),
            people: serializable_people,
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

#[get("/new-post/<title>/<body>")]
pub async fn new_post(db: MyPgDatabase, title: &str, body: &str) {
    let title = title.to_string();
    let body = body.to_string();
    let _post = db.run(move |conn| create_post(conn, title, body)).await;
}

#[get("/create-person/<first>/<last>/<address>/<city>")]
pub async fn new_person(first: &str, last: &str, address: &str, city: &str) {
    let connection = &mut establish_connection();

    create_person(connection, first, last, address, city);
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
