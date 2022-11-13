use actix_web::{get, web, HttpResponse, Responder};

use crate::service;

#[get("/rick-and-morty-challenge/{character_id}")]
pub async fn rick_and_morty_controller(path: web::Path<String>) -> impl Responder {
    let character_id = path.into_inner();
    println!("controller ID {}", character_id);
    let response = service::rick_and_morty_api_character(character_id);
    HttpResponse::Ok().json(response)
}