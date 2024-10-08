use rocketing::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end().to_string(); // Remove the trailing newline

    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let todo = add_todo(connection, title.clone(), body, 1).unwrap();
    println!("\nSaved draft {title} with id {}", todo.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
