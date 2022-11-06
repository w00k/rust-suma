use actix_web::{get, HttpResponse, Responder};
use crate::service;

#[get("/get-local-time")]
pub async fn get_locale_time() -> impl Responder {
    HttpResponse::Ok().json(service::get_local().to_string())
}

#[get("/get-utc-time")]
pub async fn get_utc_time() -> impl Responder {
    HttpResponse::Ok().json(service::get_utc().to_string())
}