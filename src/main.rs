// Imports
mod engines;
use std::sync::{Arc,Mutex};
use std::{thread, time};
use rand::rngs::StdRng;
use reqwest;
use reqwest::blocking::Client;
use reqwest::Proxy;
use serde_json::{self};
use serde_json::Value;
use rocket::Request;
use rocket::http::Status;
use randua;
use rand::{Rng, SeedableRng};
use log::{debug, error, info, warn};
use env_logger;

// Proxy List
const MAX_THREADS:u8 = 10;
#[macro_use] extern crate lazy_static;

lazy_static!{
    static ref PROXY_LIST: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    static ref THREAD_COUNT: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

}

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


    // with this enabled my server always crashes err 500 env_logger::init();
    
    let output_vector: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut new_items: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![word]));
    
    let mut old_items: Vec<String> = vec![];

    loop {

        old_items.clear();
        for item in &(*new_items.lock().unwrap()) {
            old_items.push(item.to_string());
        }

        new_items.lock().unwrap().clear();


        for kword in &old_items {

            if output_vector.lock().expect("! Lock is already taken").len() >= max {
                break;
            }

            // Setup reqwest client

            let mut rng: StdRng = SeedableRng::from_entropy();
            let proxy_list_len = PROXY_LIST.lock().expect("! Lock is already taken").len();
            let random_range = rng.gen_range(0..proxy_list_len);
            let proxy: Proxy = reqwest::Proxy::http(&*PROXY_LIST.lock().expect("! Lock is already taken")[random_range]).unwrap();
            
            

            for  engine  in engines::ENGINES.clone(){
                if output_vector.lock().expect("! Lock is already taken").len() >= max {
                    break;
                }


               
                let engine = Arc::new(Mutex::new(engine)).clone();
                let kword = Arc::new(Mutex::new(kword.to_string())).clone();
                let output_vector  = output_vector.clone();
                let new_items = new_items.clone();
                let proxy = proxy.clone();
                while *THREAD_COUNT.lock().unwrap() >= MAX_THREADS{
                    // wait thread to free
                }
                thread::spawn(move||{
                    *THREAD_COUNT.lock().unwrap()+=1;
                    println!("new thread opened currently runing  {}",*THREAD_COUNT.lock().unwrap());
                    let client:Client = reqwest::blocking::Client::builder()
                    .user_agent(randua::new().to_string())
                    .proxy(proxy.clone())
                    .build()
                    .expect("! could not build");

                   
                   
                    let client:Client = reqwest::blocking::Client::builder()
                .user_agent(randua::new().to_string())
                .proxy(proxy)
                .build()
                .expect("! could not build");
                    
                    engine.lock().unwrap()(client,kword,output_vector,new_items);
                    *THREAD_COUNT.lock().unwrap()-=1;
                    
                    println!("thread closed currently runing  {}",*THREAD_COUNT.lock().unwrap());
                });
            }



        }

        if output_vector.lock().expect("! Lock is already taken").len() >= max {
            break;
        }
    }

    output_vector.lock().expect("! Lock is already taken").truncate(max);

    let mut output_string = String::from("");

    for item in output_vector.lock().expect("! Lock is already taken").iter() {
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
                                let mut guard = PROXY_LIST.lock().expect("! Lock is already taken");
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