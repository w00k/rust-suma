use chrono::{DateTime, Local, Utc};

use crate::models::rick_and_morty_character::Character;

mod sum_service;
mod timenow_service;
mod rick_and_morty_service;

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

pub fn rick_and_morty_api_character(character_id: String) -> Character {
    println!("mopd service ID {}", character_id);
    rick_and_morty_service::rick_and_morty_call_api_character(character_id)
}