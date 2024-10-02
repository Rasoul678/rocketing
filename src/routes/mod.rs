use diesel::PgConnection;
use rocket_sync_db_pools::database;

mod api_routes;
mod tera_routes;

#[database("my_pg_db")]
pub struct MyPgDatabase(PgConnection);

pub use tera_routes::{
    create_todo_action, create_todo_view, default, index, internal_error, not_found, todos,
    update_todo_action, update_todo_view,
};

pub use api_routes::{complete, remove};
