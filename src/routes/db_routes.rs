use crate::{establish_connection, models::*};
use diesel::prelude::*;
use rocket::*;
use rocket_dyn_templates::{context, Template};

#[get("/users")]
pub async fn users() -> Template {
    use crate::schema::persons::dsl::*;
    use rocket::serde::Serialize;

    let connection = &mut establish_connection();

    let people: Vec<Person> = persons
        .limit(5)
        .select(Person::as_select())
        .load(connection)
        .expect("Error loading persons");

    #[derive(Serialize)]
    struct SerializablePerson {
        firstname: Option<String>,
        lastname: Option<String>,
        city: Option<String>,
    }

    let serializable_people: Vec<SerializablePerson> = people
        .into_iter()
        .map(|person| SerializablePerson {
            firstname: person.firstname,
            lastname: person.lastname,
            city: person.city,
        })
        .collect();

    Template::render(
        "tera/users",
        context! {
            title: "Users",
            name: Some("Rasoul"),
            people: serializable_people,
        },
    )
}
