use std::error::Error;

use reqwest::blocking::Client;
use scraper::{Selector, ElementRef, Html};

use crate::{helpers::data_helper::{SUBSCRIBERS, FIRST_SCRAPES_ISKANJEDELA, OBSERVED_LISTINGS_ISKANJEDELA}, models::listing::Listing};

use super::scrape_iskanjedela::scrape_iskanjedela;


pub fn scrape() -> Result<(), Box<dyn Error + Send + Sync>> {
        let listings_iskanjedela_reuslt = scrape_iskanjedela();
        // let sub_id = *subscriber;
        // tokio::task::spawn(async move {
        //     for sale in notification_sales {
        //         let location = match sale.sale_location {
        //             Some(l) => String::from(l),
        //             None => String::from("Unknown location")
        //         };
        //         let price = match sale.sale_price {
        //             Some(l) => String::from(l),
        //             None => String::from("Unknown price")
        //         };
        //         let href = match sale.sale_href {
        //             Some(l) => String::from(l),
        //             None => String::from("Unknown link")
        //         };    
        //         let size = match sale.sale_size {
        //             Some(l) => String::from(l),
        //             None => String::from("Unknown size")
        //         };  
        //         match Bot::from_env().send_message(
        //             teloxide::prelude::ChatId(sub_id),
        //             format!("{}:\n\t{}\n\t{}\n{}", location, price, size, href)
        //         ).await {
        //             Ok(e) => println!("{:?}", e),
        //             Err(e) => println!("{:?}", e),
        //         };
        //     }
        // });    
    Ok(())
}


fn filter_to_notify(subscriber: &i64, sales: Vec<Listing>) -> Vec<Listing> {
    let mut sales_to_notify: Vec<Listing> = Vec::new();
    let mut seen = OBSERVED_LISTINGS_ISKANJEDELA.lock().unwrap();
    let sales_ids: Vec<String> = sales.iter().map(|sale| {
        match &sale.listing_id {
            Some(id) => String::from(id),
            None => String::from("missing")
        }
    }).collect();
    match seen.get_mut(subscriber) {
        Some(seen_by_sub) => {
            println!("\t\t-> Some sales have beed seen before. Checking for new ones.");
            for sale in sales {
                let sale_id = match &sale.listing_id {
                    Some(id) => String::from(id),
                    None => String::from("missing")
                };
                if !seen_by_sub.contains(&sale_id) {
                    sales_to_notify.push(sale);
                    seen_by_sub.push(sale_id);
                }
            }
        },
        None => {
            println!("\t\t-> Sales have never been seen before. Ignoring first batch to avoid spam.");
            seen.insert(*subscriber, sales_ids);
        },
    }
    sales_to_notify
}


fn scrape_url(url: &str) -> Vec<Listing> {
    let mut next_page = true;
    let mut next_page_to_scrape = String::from(url);
    let mut sales = Vec::new();
    while next_page {
        match fetch_page(next_page_to_scrape.clone()) {
            Ok(html) => {
                let selector = Selector::parse(r#"div[itemprop="item"]"#).unwrap();
                for sale in html.select(&selector) {
                    let listing_id = get_id(sale);
                    let listing_location = get_location(sale);
                    let listing_href = get_href(sale);
                    let listing_price = get_price(sale);
                    let listing_title = get_size(sale);
                    sales.push(Listing{ 
                        listing_id,
                        listing_location,
                        listing_title, 
                        listing_href, 
                    });
                }
                // is there a next page?
                next_page = has_next_page(&html);
                if next_page {
                    next_page_to_scrape = match get_next_page_href(&html) {
                        Some(a) => a,
                        None => String::from("")
                    };
                }
            },
            Err(e) => { println!("Error scraping url HTML: {:?}", e); }
        };
    }
    sales
}


fn get_price(sale: ElementRef) -> Option<String> {
    let price_selector = Selector::parse(r#"span[class="cena"]"#).unwrap();
    for price_dom in sale.select(&price_selector) {
        return Some(price_dom.inner_html());
    }
    None
}

fn get_href(sale: ElementRef) -> Option<String> {
    let href_selector = Selector::parse(r#"h2[itemprop="name"]"#).unwrap();
    for href_dom in sale.select(&href_selector) {
        return match href_dom.value().attr("data-href") {
            Some(e) => Some(String::from("https://www.nepremicnine.net") + e),
            None => None
        };
    }
    None
}

fn get_location(sale: ElementRef) -> Option<String> {
    let location_selector = Selector::parse(r#"span[class="title"]"#).unwrap();
    for location_dom in sale.select(&location_selector) {
        return Some(location_dom.inner_html());
    }
    None
}

fn get_size(sale: ElementRef) -> Option<String> {
    let size_selector = Selector::parse(r#"span[class="velikost"]"#).unwrap();
    for size_dom in sale.select(&size_selector) {
        return Some(size_dom.inner_html());
    }
    None
}

fn get_id(sale: ElementRef) -> Option<String> {
    let id_selector = Selector::parse(r#"h2[itemprop="name"]"#).unwrap();
    for id_containing_dom in sale.select(&id_selector) {
        return match id_containing_dom.value().attr("data-href") {
            Some(e) => {
                let split = e.split("_");
                match split.last() {
                    Some(s) => Some(String::from(s)),
                    None => None
                }
            },
            None => None
        };
    }
    None
}

fn fetch_page(url: String) -> Result<Html, reqwest::Error>{
    let client = Client::builder().build().unwrap();
    match get_page_text(client, url) {
        Ok(s) => Ok(Html::parse_document(&s)),
        Err(e) => {
            println!("Error getting page text: {:?}", e);
            Err(e)
        }
    }
    
}

fn get_page_text(client: Client, url: String) -> Result<String, reqwest::Error> {
    client.get(url).send()?.text()
}

fn get_next_page_href(html: &Html) -> Option<String> {
    let next_page_selector = Selector::parse(r#"a[class="next"]"#).unwrap();
    if has_next_page(html) {
        for next_page_button_ref in html.select(&next_page_selector) {
            return match next_page_button_ref.value().attr("href") {
                Some(e) => Some(String::from("https://www.nepremicnine.net") + e),
                None => None
            };
        }
    }
    None
}

fn has_next_page(html: &Html) -> bool {
    let next_page_selector = Selector::parse(r#"a[class="next"]"#).unwrap();
    let next_page_button_count = html.select(&next_page_selector).count();
    next_page_button_count > 0
}
