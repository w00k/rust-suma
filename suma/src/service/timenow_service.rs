use chrono::prelude::*;

pub fn get_locale_time() -> DateTime<Local> {
    Local::now()
}

pub fn get_utc_time() -> DateTime<Utc>{
    Utc::now()
}