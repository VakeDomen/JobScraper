use reqwest::Error;

use crate::models::listing::Listing;

pub fn scrape_iskanjedela() -> Result<Vec<Listing>, Error> {
    Ok(vec![])
}