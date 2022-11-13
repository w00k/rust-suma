
use reqwest::blocking::{Response};
use std::{time::Duration};
use serde::{Deserialize, Serialize, de::value};

use crate::models::rick_and_morty_character::Character;

const URL_CHARACTER: &'static str = "https://rickandmortyapi.com/api/character/";

pub fn rick_and_morty_call_api_character(character_id: String) -> Character {
    println!("service ID {}", character_id);
    /*
    let url = format!("{}{}", URL_CHARACTER, character_id);
    let client = reqwest::Client::new();

    let response = client
    .get(&url)
    .header("Accept", "application/json")
    .timeout(Duration::from_secs(3))
    .send()
    ;

    Ok(())
    */
    let url = URL_CHARACTER;
    //let response = call_api_backend_sync_response_struct(character_id, url.to_string());
    let response = call_api_backend_sync_response_json(character_id, url.to_string());
    //println!("{}", response.status());
    println!("antes de transformar el Character");
    //let mut object:Character = serde_json::from_str(&response.text().unwrap()).unwrap();
    //let json_value: Character = serde_json::from_str(&response.text().unwrap()).unwrap();
    println!("{:?}", response);
    println!("posterior al Character");

    return response;
}

fn call_api(character_id: String, url: String) -> &'static str {
    let url_string = format!("{}{}", URL_CHARACTER, character_id);
    let client = reqwest::blocking::Client::new();

    println!("url : {}", url_string);

    let response = client
        .get(&url_string)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .send();

    let opt = Some(response);
    match opt {
        Some(p) => println!("has value "),
        None => println!("has no value"),
    }
    //    println!("Got {}", Some(response));

    return "ok";
}

fn call_api_backend(character_id: String, url: String) -> &'static str {
    let url_string = format!("{}{}", URL_CHARACTER, character_id);
    println!("URL : {}", url_string);

    let http_response = reqwest::blocking::get(&url_string).unwrap().text().unwrap();
    println!("{:#?}", http_response);
    return "OK";
}

fn call_api_backend_sync(character_id: String, url: String) -> &'static str {
    let url_string = format!("{}{}", URL_CHARACTER, character_id);
    println!("URL : {}", url_string);

     std::thread::spawn(move || {
        let response = reqwest::blocking::get(&url_string).unwrap();
        println!("{}", response.status());
        println!("{}", response.text().unwrap());
    }).join().unwrap();

    return "OK";
}


fn call_api_backend_sync_response_struct(character_id: String, url: String) -> Response {
    let url_string = format!("{}{}", URL_CHARACTER, character_id);
    println!("URL : {}", url_string);

    /* 
    std::thread::spawn(move || {
        let response = reqwest::blocking::get(&url_string).unwrap();
        println!("{}", response.status());
        println!("{}", response.text().unwrap());
    }).join().unwrap();
    */

    let response = std::thread::spawn(move || {reqwest::blocking::get(&url_string).unwrap()
    }).join().unwrap();

    return response;
}

fn call_api_backend_sync_response_json(character_id: String, url: String) -> Character {
    let url_string = format!("{}{}", URL_CHARACTER, character_id);
    println!("URL : {}", url_string);

    /* 
    std::thread::spawn(move || {
        let response = reqwest::blocking::get(&url_string).unwrap();
        println!("{}", response.status());
        println!("{}", response.text().unwrap());
    }).join().unwrap();
    */

    let response = std::thread::spawn(move || {
        let resp = reqwest::blocking::get(&url_string).unwrap();
        let json_value: Character = serde_json::from_str(&resp.text().unwrap_or_default()).unwrap();
        return json_value;
    }).join().unwrap();

    return response;
}

async fn call_async(character_id: String, url: String) -> &'static str {
    let url = format!("{}{}", URL_CHARACTER, character_id);
    let client = reqwest::Client::new();

    let response = client
    .get(&url)
    .header("Accept", "application/json")
    .timeout(Duration::from_secs(3))
    .send()
    ;

    return "OK";
}

async fn call_sync_i_think(character_id: String, url: String) -> &'static str {
    let url = format!("{}{}", URL_CHARACTER, character_id);
    let client = reqwest::Client::new();

    let response = reqwest::get(&url).await.unwrap().text().await.unwrap();

    println!("{:?}", response);

    return "OK";
}