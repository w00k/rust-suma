use actix_web::{ App, HttpServer, web::service};

mod service;
pub mod controller;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addrs = ("127.0.0.1", 8080);

    HttpServer::new(move || {
        App::new()
            .service(controller::hello_controller::hello)
            .service(controller::time_controller::get_locale_time)
            .service(controller::time_controller::get_utc_time)
            .service(controller::suma_controller::do_sum)
            .service(controller::suma_controller::do_sum_var)
    })
    .bind(addrs)?
    .run()
    .await
}
