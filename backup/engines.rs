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

  pub fn bing(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
 
 // Bing Scraping

 let bing_url: String = format!("https://api.bing.com/osjson.aspx?query={}", kword);

 match client.get(&bing_url).send(){
     Ok(response) => {
         if response.status() == reqwest::StatusCode::OK {
             match response.text(){
                 Ok(text) => {
                     let value: Value = serde_json::from_str(&text).expect("! Problem reading bing response");
                     let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Bing changed their JSON response");
                     items.into_iter().for_each(|item| {
                         if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                             output_vector.lock().expect("! Lock is already taken").push(item.clone());
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

 }

// ======================================================================================================================




// ======================================================================================================================

 pub fn yep(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

    let yep_url: String = format!("https://api.yep.com/ac/?query={}", kword);

         match client.get(&yep_url).send() {
             Ok(response) => {
                 if response.status() == reqwest::StatusCode::OK {
                     match response.text() {
                         Ok(text) => {
                             let value: Value = serde_json::from_str(&text).expect("! Problem reading yep response");
                             let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! yep changed their JSON response");
                             items.into_iter().for_each(|item| {
                                 if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                     output_vector.lock().expect("! Lock is already taken").push(item.clone());
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
    
       
   
    }



// ======================================================================================================================










// ======================================================================================================================

pub fn ask(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
let ask_url: String = format!("https://amg-ss.ask.com/query?lang=en-US&limit=20&q={}", kword);

     match client.get(&ask_url).send() {
         Ok(response) => {
             if response.status() == reqwest::StatusCode::OK {
                 match response.text() {
                     Ok(text) => {
                         let value: Value = serde_json::from_str(&text).expect("! Problem reading ask.com response");
                         let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Ask.com changed their JSON response");
                         items.into_iter().for_each(|item| {
                             if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                 output_vector.lock().expect("! Lock is already taken").push(item.clone());
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
    }


// ======================================================================================================================










// ======================================================================================================================


    pub fn neeva(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
     let neeva_url: String = format!("https://neeva.com/suggest?q={}", kword);

     match client.get(&neeva_url).send() {
         Ok(response) => {
             if response.status() == reqwest::StatusCode::OK {
                 match response.text() {
                     Ok(text) => {
                         let value: Value = serde_json::from_str(&text).expect("! Problem reading neeva response");
                         let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! Neeva changed their JSON response");
                         items.into_iter().for_each(|item| {
                             if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                 output_vector.lock().expect("! Lock is already taken").push(item.clone());
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

    }


// ======================================================================================================================
pub fn seznam(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
     let seznam_url: String = format!("https://suggest.seznam.cz/fulltext/cs?phrase={}&count=100", kword);

     match client.get(&seznam_url).send() {
         Ok(response) => {
             if response.status() == reqwest::StatusCode::OK {
                 match response.text() {
                     Ok(text) => {
                         let value: Value = serde_json::from_str(&text).expect("! Problem reading seznam response");
                         let items: Vec<Value> = serde_json::from_value(value["result"].to_owned()).expect("! seznam changed their JSON response");
                         let items:Vec<String> = items.into_iter().map(|v|{
                             return serde_json::from_value(v["text"][0]["text"].to_owned()).expect("! seznam changed their JSON response maping Failed!!!" );
                            
                         }).collect();
                         
                         items.into_iter().for_each(|item| {
                             if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                 output_vector.lock().expect("! Lock is already taken").push(item.clone());
                             }
                             if ! new_items.contains(&item) {
                                 new_items.push(item);
                             }
                         })
                     }
                     Err(_) => println!("! Could not read seznam response json ")
                 }
             }
         }
         Err(_) => println!("! seznam request Error")
     }
    }


// ======================================================================================================================





// ======================================================================================================================
pub fn duckduckgo(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

         let duckduckgo_url: String = format!("https://duckduckgo.com/ac/?q={}", kword);

         match client.get(&duckduckgo_url).send() {
            
             Ok(response) => {
               
                 if response.status() == reqwest::StatusCode::OK {
                     match response.text() {
                         Ok(text) => {
                             let value: Vec<Value> = serde_json::from_str(&text).expect("! Problem reading duckduckgo response");

                             let items:Vec<String> = value.into_iter().map(|v|{
                                 return serde_json::from_value(v["phrase"].to_owned()).expect("! duckduckgo changed their JSON response maping Failed!!!" );
                                 
                             }).collect();
                             
                             items.into_iter().for_each(|item| {
                                 if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                     output_vector.lock().expect("! Lock is already taken").push(item.clone());
                                 }
                                 if ! new_items.contains(&item) {
                                     new_items.push(item);
                                 }
                             })
                         }
                         Err(_) => println!("! Could not read duckduckgo response json ")
                     }
                 }
             }
             Err(_) => println!("! duckduckgo request Error")
         }

   
}
// ======================================================================================================================












// ======================================================================================================================
  


pub fn yahoo(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
       // yahoo scraping
       let yahoo_url: String = format!("https://search.yahoo.com/sugg/gossip/gossip-us-fastbreak/?pq=&command={}&output=sd1&nresults=20", kword);

       match client.get(&yahoo_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading yahoo response");
                           let items: Vec<Value> = serde_json::from_value(value["r"].to_owned()).expect("! yahoo changed their JSON response");
                           let items:Vec<String> = items.into_iter().map(|v|{
                               return serde_json::from_value(v["k"].to_owned()).expect("! yahoo changed their JSON response maping Failed!!!" );
                               
                           }).collect();
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read yahoo response json ")
                   }
               }
           }
           Err(_) => println!("! yahoo request Error")
       }
      

}
// ======================================================================================================================
pub fn etsy(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
       
        // etsy scraping
        let etsy_url: String = format!("https://www.etsy.com/suggestions_ajax.php?search_query={}", kword);

        match client.get(&etsy_url).send() {
           
            Ok(response) => {
              
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(text) => {
                            let value: Value = serde_json::from_str(&text).expect("! Problem reading etsy response");
                            let items: Vec<Value> = serde_json::from_value(value["results"].to_owned()).expect("! etsy changed their JSON response");
                            let items:Vec<String> = items.into_iter().map(|v|{
                                return serde_json::from_value(v["query"].to_owned()).expect("! etsy changed their JSON response maping Failed!!!" );
                                
                            }).collect();
                            
                            items.into_iter().for_each(|item| {
                                if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                    output_vector.lock().expect("! Lock is already taken").push(item.clone());
                                }
                                if ! new_items.contains(&item) {
                                    new_items.push(item);
                                }
                            })
                        }
                        Err(_) => println!("! Could not read etsy response json ")
                    }
                }
            }
            Err(_) => println!("! etsy request Error")
        }
       
}
// ======================================================================================================================




// ======================================================================================================================        
        // ebay scraping
pub fn ebay(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
        let ebay_url: String = format!("https://www.ebay.com/autosug?kwd={}&sId=0&callback=0", kword);

        match client.get(&ebay_url).send() {
           
            Ok(response) => {
              
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(text) => {
                            let value: Value = serde_json::from_str(&text).expect("! Problem reading ebay response");
                            let items: Vec<String> = serde_json::from_value(value["res"]["sug"].to_owned()).expect("! ebay changed their JSON response");
                           
                           
                            
                            items.into_iter().for_each(|item| {
                                if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                    output_vector.lock().expect("! Lock is already taken").push(item.clone());
                                }
                                if ! new_items.contains(&item) {
                                    new_items.push(item);
                                }
                            })
                        }
                        Err(_) => println!("! Could not read ebay response json ")
                    }
                }
            }
            Err(_) => println!("! ebay request Error")
        }
       
    }
// ======================================================================================================================

// ======================================================================================================================
     // yandex Scraping
 pub fn yandex(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

     let yandex_url: String = format!("https://yandex.com/suggest/suggest-ya.cgi?n=100&part={}", kword);

     match client.get(&yandex_url).send() {
         Ok(response) => {
             if response.status() == reqwest::StatusCode::OK {
                 match response.text() {
                     Ok(text) => {
                       
                         let value: Value = serde_json::from_str(&text[14..text.len()-4]).expect("! Problem reading yandex response");
                         let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! yandex changed their JSON response");
                        
                         
                         items.into_iter().for_each(|item| {
                             if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                 output_vector.lock().expect("! Lock is already taken").push(item.clone());
                             }
                             if ! new_items.contains(&item) {
                                 new_items.push(item);
                             }
                         })
                     }
                     Err(_) => println!("! Could not read yandex response json ")
                 }
             }
         }
         Err(_) => println!("! yandex request Error")
     }
    }


// ======================================================================================================================


// ======================================================================================================================
pub fn naver(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
    // naver Scraping

     let naver_url: String = format!("https://ac.search.naver.com/nx/ac?q={}&st=100", kword);

     match client.get(&naver_url).send() {
         Ok(response) => {
             if response.status() == reqwest::StatusCode::OK {
                 match response.text() {
                     Ok(text) => {
                       
                         let value: Value = serde_json::from_str(&text).expect("! Problem reading naver response");
                         let items: Vec<Vec<String>> = serde_json::from_value(value["items"][0].to_owned()).expect("! naver changed their JSON response");
                        
                         
                         items.into_iter().for_each(|item| {
                             let item = item[0].to_owned();
                             if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                 output_vector.lock().expect("! Lock is already taken").push(item.clone());
                             }
                             if ! new_items.contains(&item) {
                                 new_items.push(item);
                             }
                         })
                     }
                     Err(_) => println!("! Could not read naver response json ")
                 }
             }
         }
         Err(_) => println!("! naver request Error")
     }

}
// ======================================================================================================================
// ======================================================================================================================
pub fn aol(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
       // aol scraping
       let aol_url: String = format!("https://search.aol.com/sugg/gossip/gossip-us-ura/?command={}&output=sd1", kword);

       match client.get(&aol_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading aol response");
                           let items: Vec<Value> = serde_json::from_value(value["r"].to_owned()).expect("! aol changed their JSON response");
                           let items:Vec<String> = items.into_iter().map(|v|{
                               return serde_json::from_value(v["k"].to_owned()).expect("! aol changed their JSON response maping Failed!!!" );
                               
                           }).collect();
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read aol response json ")
                   }
               }
           }
           Err(_) => println!("! aol request Error")
       }
      
    }      

// ======================================================================================================================




// ======================================================================================================================
pub fn amazon(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

       // amazon scraping
       let amazon_url: String = format!("https://completion.amazon.com/api/2017/suggestions?limit=11&prefix={}&alias=aps&mid=ATVPDKIKX0DER", kword);

       match client.get(&amazon_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading amazon response");
                           let items: Vec<Value> = serde_json::from_value(value["suggestions"].to_owned()).expect("! amazon changed their JSON response");
                           let items:Vec<String> = items.into_iter().map(|v|{
                               return serde_json::from_value(v["value"].to_owned()).expect("! amazon changed their JSON response maping Failed!!!" );
                               
                           }).collect();
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read amazon response json ")
                   }
               }
           }
           Err(_) => println!("! amazon request Error")
       }
      
    }      

// ======================================================================================================================
// ======================================================================================================================
pub fn swisscows(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

       // swisscows scraping
       let swisscows_url: String = format!("https://swisscows.com/api/suggest?query={}&region=en-US&itemsCount=20", kword);

       match client.get(&swisscows_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading swisscows response");
                           let items: Vec<String> = serde_json::from_value(value.to_owned()).expect("! swisscows changed their JSON response");
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read swisscows response json ")
                   }
               }
           }
           Err(_) => println!("! swisscows request Error")
       }
    }
      

// ======================================================================================================================
// ======================================================================================================================
pub fn ecosia(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

       // ecosia scraping
       let ecosia_url: String = format!("https://ac.ecosia.org/?q={}&mkt=en-us", kword);

       match client.get(&ecosia_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading ecosia response");
                           let items: Vec<String> = serde_json::from_value(value["suggestions"].to_owned()).expect("! ecosia changed their JSON response");
                         
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read ecosia response json ")
                   }
               }
           }
           Err(_) => println!("! ecosia request Error")
       }
    }      
      
// ======================================================================================================================
// ======================================================================================================================
pub fn wolframalpha(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();

       // wolframalpha scraping only works if the word is very small. not work gaming work ga
       let wolframalpha_url: String = format!("https://www.wolframalpha.com/n/v1/api/autocomplete/?i={}", kword);

       match client.get(&wolframalpha_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading wolframalpha response");
                           let items: Vec<Value> = serde_json::from_value(value["results"].to_owned()).expect("! wolframalpha changed their JSON response");
                           let items:Vec<String> = items.into_iter().map(|v|{
                               return serde_json::from_value(v["input"].to_owned()).expect("! wolframalpha changed their JSON response maping Failed!!!" );
                               
                           }).collect();
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read wolframalpha response json ")
                   }
               }
           }
           Err(_) => println!("! wolframalpha request Error")
       }
      
    }

// ======================================================================================================================

// ======================================================================================================================
pub fn qwant(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
       // qwant scraping
       let qwant_url: String = format!("https://api.qwant.com/v3/suggest?q={}", kword);

       match client.get(&qwant_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                         println!("{}", text);
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading qwant response");
                           let items: Vec<Value> = serde_json::from_value(value["data"]["items"].to_owned()).expect("! qwant changed their JSON response");
                           let items:Vec<String> = items.into_iter().map(|v|{
                               return serde_json::from_value(v["value"].to_owned()).expect("! qwant changed their JSON response maping Failed!!!" );
                               
                           }).collect();
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read qwant response json ")
                   }
               }
           }
           Err(_) => println!("! qwant request Error")
       }
      
    }
// ======================================================================================================================
// ======================================================================================================================
pub fn you(client:Client,kword:Arc<Mutex<String>>,output_vector:Arc<Mutex<Vec<String>>>,new_items:Arc<Mutex<Vec<String>>>,thread_count:Arc<Mutex<u8>>){
    let kword =  kword.lock().unwrap();
    let mut  new_items =new_items.lock().unwrap();
       // you scraping 
       let you_url: String = format!("https://you.com/api/ac?q={}", kword);

       match client.get(&you_url).send() {
          
           Ok(response) => {
             
               if response.status() == reqwest::StatusCode::OK {
                   match response.text() {
                       Ok(text) => {
                         println!("{}", text);
                           let value: Value = serde_json::from_str(&text).expect("! Problem reading you response");
                           let items: Vec<String> = serde_json::from_value(value[1].to_owned()).expect("! you changed their JSON response");
                         
                           
                           items.into_iter().for_each(|item| {
                               if ! output_vector.lock().expect("! Lock is already taken").contains(&item) {
                                   output_vector.lock().expect("! Lock is already taken").push(item.clone());
                               }
                               if ! new_items.contains(&item) {
                                   new_items.push(item);
                               }
                           })
                       }
                       Err(_) => println!("! Could not read you response json ")
                   }
               }
           }
           Err(_) => println!("! you request Error")
       }
      
}
// ======================================================================================================================
