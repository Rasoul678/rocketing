use crate::{complete_todo, delete_todo};
use rocket::http::Status;
use rocket::*;

use super::MyPgDatabase;

#[delete("/todos/delete/<id>")]
async fn remove(db: MyPgDatabase, id: i32) -> Status {
    let deleted_todo = db.run(move |conn| delete_todo(conn, id)).await;

    match deleted_todo {
        Ok(num) => {
            if num == 1 {
                Status::Accepted
            } else {
                Status::NotAcceptable
            }
        }
        Err(_) => Status::InternalServerError,
    }
}

#[patch("/todos/complete/<id>")]
async fn complete(db: MyPgDatabase, id: i32) -> Status {
    let todo = db.run(move |conn| complete_todo(conn, id)).await;

    match todo {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![remove, complete]
}
