use chrono::{DateTime, Local, Utc};

mod sum_service;
mod timenow_service;

pub fn get_local() -> DateTime<Local> {
    timenow_service::get_locale_time()
}

pub fn get_utc() -> DateTime<Utc> {
    timenow_service::get_utc_time()
}

pub fn suma_normal(x: i32, y:i32) -> i32 {
    sum_service::suma(x, y)
} 

pub fn suma_variadica(x:i32, y:i32) -> i32 {
    sum_service::suma_variadica(x, y)
}