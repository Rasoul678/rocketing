use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::FromForm;
use serde::Deserialize;

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
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: usize,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthUser {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|user_id| AuthUser { user_id })
            .or_forward(Status::Unauthorized)
    }
}
