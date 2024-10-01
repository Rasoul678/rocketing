mod db_routes;
mod tera_routes;

pub use db_routes::{
    create_todo, new_person, new_post, new_todo, todos, update_todo, users, MyPgDatabase,
};
pub use tera_routes::{default, index, internal_error, not_found};
