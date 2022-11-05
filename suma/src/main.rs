use chrono::prelude::*;

pub mod service;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let now = Local::now();
    println!("Start at {} local datetime, {} ", now, utc);

    println!("{}", service::suma::suma(10, 4));
    println!("{}", service::suma::suma_variadica(10, 4));
}
