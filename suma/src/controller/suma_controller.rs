use actix_web::{ post, web, HttpResponse, Responder};
use crate::models::sum_request::SumReq;
use crate::models::sum_response::SumRes;
use crate::service;

//use self::models::{Sum_req, Sum_res};

#[post("/do-sum")]
pub async fn do_sum(req: web::Json<SumReq>) -> impl Responder {
    let x = req.x;
    let y = req.y;
    let respuesta = service::suma_normal(x, y);
    let response = SumRes {
        resp: respuesta
    };
    HttpResponse::Ok().json(response)
}

#[post("/do-sum-var")]
pub async fn do_sum_var(req: web::Json<SumReq>) -> impl Responder {
    let x = req.x;
    let y = req.y;
    HttpResponse::Ok().json(
        service::suma_variadica(x, y)
    )
}