use diesel::prelude::*;
use rocketing::*;
use std::env::args;

fn main() {
    use rocketing::schema::todos::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(todos.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting todos");

    println!("Deleted {} todos", num_deleted);
}
