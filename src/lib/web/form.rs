use crate::domain::clip::field;
use rocket::form::FromForm;
use serde::Serialize;

#[derive(Debug, Serialize, FromForm)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub password: field::Password,
    pub expires: field::Expires,
}

#[derive(Debug, Serialize, FromForm)]
pub struct GetPasswordProtectedClip {
    pub password: field::Password,
}
