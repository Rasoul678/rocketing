use diesel::prelude::*;
use crate::schema::*;

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
    pub lastname: Option<String>,
    pub firstname: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
}
