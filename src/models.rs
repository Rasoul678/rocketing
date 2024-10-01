use crate::schema::*;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
