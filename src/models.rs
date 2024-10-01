use crate::schema::*;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = persons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = persons)]
pub struct NewPerson<'a> {
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
    pub address: Option<&'a str>,
    pub city: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
