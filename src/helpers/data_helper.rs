use std::sync::Mutex;

use once_cell::sync::Lazy;

const SUBSCRIBERS_PATH: &str = "subscribers.json";
const OBSERVED_LISTINGS_ISKANJEDELA_PATH: &str = "listings_iskanjedela.json";

pub static SUBSCRIBERS: Lazy<Mutex<Vec<i64>>> = Lazy::new(|| {
    match serde_any::from_file(SUBSCRIBERS_PATH) {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(Vec::new())
    }
});

pub static OBSERVED_LISTINGS_ISKANJEDELA: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    match serde_any::from_file(OBSERVED_LISTINGS_ISKANJEDELA_PATH) {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(Vec::new())
    }
});

pub fn save_subscribers() {
    let subs = SUBSCRIBERS.lock().unwrap();
    match serde_any::to_file(SUBSCRIBERS_PATH, &*subs) {
        Ok(_) => {},
        Err(e) => {println!("Error saving SUBSCRIBERS: {:?}", e);}
    };
}

pub fn save_observed_listings_iskanjedela() {
    let subs = OBSERVED_LISTINGS_ISKANJEDELA.lock().unwrap();
    match serde_any::to_file(OBSERVED_LISTINGS_ISKANJEDELA_PATH, &*subs) {
        Ok(_) => {},
        Err(e) => {println!("Error saving OBSERVED_LISTINGS_ISKANJEDELA: {:?}", e);}
    };
}