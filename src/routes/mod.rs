use diesel::PgConnection;
use rocket_sync_db_pools::database;

mod api_routes;
mod tera_routes;

#[database("my_pg_db")]
pub struct MyPgDatabase(PgConnection);

pub use tera_routes::{
    create_todo, default, index, internal_error, new_todo, not_found, todos, update_todo,
};

pub use api_routes::{complete, remove};
