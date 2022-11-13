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
            .service(controller::hello_controller::hello_user_name)
            .service(controller::time_controller::get_locale_time)
            .service(controller::time_controller::get_utc_time)
            .service(controller::sum_controller::do_sum)
            .service(controller::sum_controller::do_sum_var)
            .service(controller::rick_and_morty_controller::rick_and_morty_controller)
    })
    .bind(addrs)?
    .run()
    .await
}
