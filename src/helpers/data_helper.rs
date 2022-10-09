use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;

const SUBSCRIBERS_PATH: &str = "subscribers.json";

pub static SUBSCRIBERS: Lazy<Mutex<Vec<i64>>> = Lazy::new(|| {
    match serde_any::from_file(SUBSCRIBERS_PATH) {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(Vec::new())
    }
});

pub static OBSERVED_LISTINGS_ISKANJEDELA: Lazy<Mutex<HashMap<i64, Vec<String>>>> = Lazy::new(|| {
    match serde_any::from_file("listings_iskanjedela.json") {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(HashMap::new())
    }
});

pub static FIRST_SCRAPES_ISKANJEDELA: Lazy<Mutex<HashMap<i64, Vec<String>>>> = Lazy::new(|| {
    match serde_any::from_file("first_scrapes_iskanjedela.json") {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(HashMap::new())
    }
});

pub fn save_subscribers() {
    let subs = SUBSCRIBERS.lock().unwrap();
    match serde_any::to_file(SUBSCRIBERS_PATH, &*subs) {
        Ok(_) => {();},
        Err(e) => {println!("Error saving subscirbers: {:?}", e);}
    };
}