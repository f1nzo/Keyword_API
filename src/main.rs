// Imports
use std::sync::{Arc, RwLock};
use std::{thread, time};
use rand::rngs::StdRng;
use reqwest;
use reqwest::{Proxy, Client};
use serde_json::{self};
use serde_json::Value;
use rocket::Request;
use rocket::http::Status;
use randua;
use rand::{Rng, SeedableRng};
use log::{debug, error, info, warn};
use env_logger;

// Proxy List

#[macro_use] extern crate lazy_static;

lazy_static!(static ref PROXY_LIST: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![])););

// Rocket.rs

#[macro_use] extern crate rocket;

// Catch 500

#[catch(500)]
fn internal_error() -> String {
    return String::from("500 error");
}

// Catch 404

#[catch(404)]
fn not_found() -> String {
    return String::from("404 error");
}

// Catch other errors

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

// Home page

#[get("/")]
fn home() -> String {
    format!("Welcome to Kraken Keywords")
}

// Keyword genorator

#[get("/<word>/<max>")]
async fn gen(word: String, max: usize) -> String {

    env_logger::init();
    
    let output_vector: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));
    let mut new_items: Vec<String> = vec![];
    new_items.push(word);
    let mut old_items: Vec<String> = vec![];

    loop {

        old_items.clear();
        for item in &new_items {
            old_items.push(item.to_string());
        }

        new_items.clear();


        for kword in &old_items {

            if output_vector.read().expect("! Lock is already taken").len() >= max {
                break;
            }

            // Setup reqwest client

            let mut rng: StdRng = SeedableRng::from_entropy();
            let proxy_list_len = PROXY_LIST.read().expect("! Lock is already taken").len();
            let random_range = rng.gen_range(0..proxy_list_len);
            let proxy: Proxy = reqwest::Proxy::http(&*PROXY_LIST.read().expect("! Lock is already taken")[random_range]).unwrap();
            
            let client: Client = reqwest::ClientBuilder::new()
                .user_agent(randua::new().to_string())
                .proxy(proxy)
                .build()
                .expect("! could not build");

            // Bing Scraping

            let bing_url: String = format!("https://api.bing.com/osjson.aspx?query={}", kword);

            match client.get(&bing_url).send().await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.text().await {
                            Ok(text) => {
                                let value: Value = serde_json::from_str(&text).expect("! Problem reading bing response");
                                let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Bing changed their JSON response");
                                items.into_iter().for_each(|item| {
                                    if ! output_vector.read().expect("! Lock is already taken").contains(&item) {
                                        output_vector.write().expect("! Lock is already taken").push(item.clone());
                                    }
                                    if ! new_items.contains(&item) {
                                        new_items.push(item);
                                    }
                                })
                            }
                            Err(_) => println!("! Could not read bing response json ")
                        }
                    }
                }
                Err(_) => println!("! Bing request Error")
            }

            if output_vector.read().expect("! Lock is already taken").len() >= max {
                break;
            }


            // Yep Scraping

            let yep_url: String = format!("https://api.yep.com/ac/?query={}", kword);

            match client.get(&yep_url).send().await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.text().await {
                            Ok(text) => {
                                let value: Value = serde_json::from_str(&text).expect("! Problem reading yep response");
                                let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! yep changed their JSON response");
                                items.into_iter().for_each(|item| {
                                    if ! output_vector.read().expect("! Lock is already taken").contains(&item) {
                                        output_vector.write().expect("! Lock is already taken").push(item.clone());
                                    }
                                    if ! new_items.contains(&item) {
                                        new_items.push(item);
                                    }
                                })
                            }
                            Err(_) => println!("! Could not read yep response json ")
                        }
                    }
                }
                Err(_) => println!("! Yep request Error")
            }

            if output_vector.read().expect("! Lock is already taken").len() >= max {
                break;
            }

            // Ask.com Scraping

            let ask_url: String = format!("https://amg-ss.ask.com/query?lang=en-US&limit=20&q={}", kword);

            match client.get(&ask_url).send().await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.text().await {
                            Ok(text) => {
                                let value: Value = serde_json::from_str(&text).expect("! Problem reading ask.com response");
                                let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Ask.com changed their JSON response");
                                items.into_iter().for_each(|item| {
                                    if ! output_vector.read().expect("! Lock is already taken").contains(&item) {
                                        output_vector.write().expect("! Lock is already taken").push(item.clone());
                                    }
                                    if ! new_items.contains(&item) {
                                        new_items.push(item);
                                    }
                                })
                            }
                            Err(_) => println!("! Could not read ask.com response json ")
                        }
                    }
                }
                Err(_) => println!("! Ask.com request Error")
            }

            if output_vector.read().expect("! Lock is already taken").len() >= max {
                break;
            }

            // Neeva.com Scraping

            let neeva_url: String = format!("https://neeva.com/suggest?q={}", kword);

            match client.get(&neeva_url).send().await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.text().await {
                            Ok(text) => {
                                let value: Value = serde_json::from_str(&text).expect("! Problem reading neeva response");
                                let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Neeva changed their JSON response");
                                items.into_iter().for_each(|item| {
                                    if ! output_vector.read().expect("! Lock is already taken").contains(&item) {
                                        output_vector.write().expect("! Lock is already taken").push(item.clone());
                                    }
                                    if ! new_items.contains(&item) {
                                        new_items.push(item);
                                    }
                                })
                            }
                            Err(_) => println!("! Could not read neeva response json ")
                        }
                    }
                }
                Err(_) => println!("! Neeva request Error")
            }

            if output_vector.read().expect("! Lock is already taken").len() >= max {
                break;
            }
        }

        if output_vector.read().expect("! Lock is already taken").len() >= max {
            break;
        }
    }

    output_vector.write().expect("! Lock is already taken").truncate(max);

    let mut output_string = String::from("");

    for item in output_vector.read().expect("! Lock is already taken").iter() {
        output_string = format!("{output_string}{item}\n")
    }

    return output_string;
}

#[launch]
fn rocket() -> _ {
    thread::spawn(|| {
        loop {
            match reqwest::blocking::get("https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&timeout=1000&country=all&ssl=all&anonymity=all") {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.text() {
                            Ok(text) => {
                                let mut guard = PROXY_LIST.write().expect("! Lock is already taken");
                                *guard = text.split("\n").map(String::from).collect();
                            }
                            Err(_) => println!("! Could not read proxyscrape.com response json ")
                        }
                    }
                }
                Err(_) => println!("! Proxyscrape.com request Error")
            }
            thread::sleep(time::Duration::from_secs(60));
        }
    });
    rocket::build().mount("/", routes![home, gen]).register("/", catchers![internal_error, not_found, default])
}