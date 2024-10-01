use diesel::prelude::*;
use models::Todo;
use rocketing::*;
use std::env::args;

fn main() {
    use crate::schema::todos::dsl::{completed, todos};

    let id = args()
        .nth(1)
        .expect("complete_todo requires a todo id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post = diesel::update(todos.find(id))
        .set(completed.eq(true))
        .returning(Todo::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Todo completed {}", post.title);
}
