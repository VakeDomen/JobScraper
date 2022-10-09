use std::{error::Error, thread};

use teloxide::{Bot, requests::Requester};

use crate::helpers::data_helper::SUBSCRIBERS;

use super::scrape_iskanjedela::scrape_iskanjedela;


pub fn scrape() -> Result<(), Box<dyn Error + Send + Sync>> {
    let isaknjedela_handle = thread::spawn(|| {
        match scrape_iskanjedela() {
            Ok(data) => data,
            Err(_) => vec![]
        }
    });
    let isaknjedela_listings = match isaknjedela_handle.join() {
        Ok(data) => data,
        Err(_) => vec![],
    };
    let subs = SUBSCRIBERS.lock().unwrap();
    for sub in subs.iter() {
        for listing in isaknjedela_listings.iter() {
            let sub_id = *sub;
            let location = listing.listing_location.clone();
            let title = listing.listing_title.clone();
            let href = listing.listing_href.clone();
            send_to_telegram(sub_id, location, title, href);
        }
    }
    Ok(())
}

fn send_to_telegram(user_id: i64, location: String, title: String, href: String) {
    tokio::task::spawn(async move {
        match Bot::from_env().send_message(
            teloxide::prelude::ChatId(user_id),
            format!("{}:\n\t{}\n{}", location, title, href)
        ).await {
            Ok(e) => println!("{:?}", e),
            Err(e) => println!("{:?}", e),
        };
    });  
}