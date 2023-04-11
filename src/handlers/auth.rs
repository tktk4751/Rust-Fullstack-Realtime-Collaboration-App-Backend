use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::user::{NewUser, User};
use crate::utils::password::hash_password;

#[derive(Deserialize)]
pub struct SignupUser {
    email: String,
    password: String,
}

pub async fn signup(user_data: web::Json<SignupUser>) -> impl Responder {
    let hashed_password = match hash_password(&user_data.password) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_user = NewUser {
        email: user_data.email.clone(),
        password: hashed_password,
    };

    // TODO: Save the new_user to the database

    HttpResponse::Created().finish()
}

#[derive(Deserialize)]
pub struct SigninUser {
    email: String,
    password: String,
}

pub async fn signin(user_data: web::Json<SigninUser>) -> impl Responder {
    // TODO: Authenticate the user and generate a JWT

    HttpResponse::Ok().finish()
}
