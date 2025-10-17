use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize; 
use domain::User; 
use infrastructure::InMemoryUserRepo;
use std::sync::Arc; 
use crate::AppState;

#[derive(Deserialize)]

pub struct RegisterInput {
    username: String, 
    email: String,
    password: String, 
}

pub async fn register_user(
    data: web::Data<AppState>,
    input: web::Json<RegisterInput>,
) -> impl Responder {

    let user = User::new(&input.username, &input.email, &input.password); 

    match data.repo.save(user) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::Conflict().body("username already exists",)
    }   


}

pub async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let username = path.into_inner(); 
     match data.repo.find(&username) {
        Some(user) => HttpResponse::Ok().json(user.username),
        None => HttpResponse::NotFound().body("User not found"),
    }  
}