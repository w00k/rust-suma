use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[get("/hello/{user_name}")]
pub async fn hello_user_name(path: web::Path<String>) -> impl Responder {
    let user_name = path.into_inner();
    HttpResponse::Ok().json(format!("Hello {}", user_name))
}