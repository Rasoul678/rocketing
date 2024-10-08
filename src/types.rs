use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::serde_json;
use rocket::FromForm;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(FromForm, Debug, Deserialize)]
pub struct TodoForm {
    pub title: String,
    pub body: String,
    pub id: Option<i32>,
}

#[derive(FromForm, Debug, Deserialize)]
pub struct RegisterUserForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(FromForm, Debug, Deserialize)]
pub struct LoginUserForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthUser {
    type Error = Infallible;

    async fn from_request(req: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("user")
            .and_then(|cookie| serde_json::from_str::<AuthUser>(cookie.value()).ok())
            .map(|user| user)
            .or_forward(Status::Unauthorized)
    }
}
