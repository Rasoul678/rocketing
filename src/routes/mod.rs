use diesel::PgConnection;
use rocket_sync_db_pools::database;

mod api_routes;
mod tera_routes;

#[database("my_pg_db")]
pub struct MyPgDatabase(PgConnection);

pub use tera_routes::{catchers, tera_routes};

pub use api_routes::api_routes;
