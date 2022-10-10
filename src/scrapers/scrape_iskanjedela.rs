use reqwest::{Error, blocking::Response};

use crate::{models::{listing::Listing, response_iskanjedela::ResponseIskanjedela}, config::iskanjedela::LOCATIONS, helpers::data_helper::{OBSERVED_LISTINGS_ISKANJEDELA, save_observed_listings_iskanjedela}};

pub fn scrape_iskanjedela() -> Result<Vec<Listing>, Error> {
    let raw_responses = fetch_raw_data();
    let iskanjedela_responses = parse_listing_data(raw_responses);
    let iskanjedela_listings = to_general_listings(iskanjedela_responses);
    let filtered_listings = filter_previously_seen_listings(iskanjedela_listings);
    Ok(filtered_listings)
}

fn filter_previously_seen_listings(listings: Vec<Listing>) -> Vec<Listing> {
    let filtered_listings = {
        let mut all_listings = OBSERVED_LISTINGS_ISKANJEDELA.lock().unwrap();
        let report_to_user = !all_listings.is_empty();
        let mut to_report = vec![];
        for listing in listings.into_iter() {
            if !all_listings.contains(&listing.listing_id) {
                all_listings.push(listing.listing_id.clone());
                if report_to_user {
                    to_report.push(listing);
                }
            }
        }
        to_report
    };
    save_observed_listings_iskanjedela();
    filtered_listings
}

fn to_general_listings(iskanjedela_responses: Vec<ResponseIskanjedela>) -> Vec<Listing> {
    let mut listings = vec![];
    for resp in iskanjedela_responses.into_iter() {
        for listing in resp.data.into_iter() {
            listings.push(listing);
        }
    }
    listings.into_iter().map(Listing::from).collect()
}

fn parse_listing_data(raw: Vec<Response>) -> Vec<ResponseIskanjedela> {
    raw.into_iter()
        .filter_map(|r| r.text().ok())
        .collect::<Vec<String>>()
        .iter_mut()
        .filter_map(|s| serde_json::from_str(s.as_str()).ok())
        .collect()
}

fn fetch_raw_data() -> Vec<Response> {
    let mut responses = vec![];
    for location in LOCATIONS {
        println!("[ISKANJE DELA] fetching data for location: {}", location);
        let target_uri = format!("https://www.iskanjedela.si/api/jobs?limit=200&location={}&facets=RECENCY==TODAY", location);
        let resp_option = match reqwest::blocking::get(target_uri) {
            Ok(data) => Some(data),
            Err(e) =>  {
                println!("[ISKANJE DELA] Error fetching uri {:#?}", e.to_string());
                None
            },
        };
        match resp_option {
            Some(resp) => responses.push(resp),
            None => println!("[ISJANJE DELA] Response option in None!")
        };
    }
    responses
}