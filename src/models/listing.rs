use super::response_iskanjedela::ListingIskanjedela;

#[derive(Clone, Debug)]
pub struct Listing {
    pub listing_id: String,
    pub listing_title: String,
    pub listing_location: String,
    pub listing_href: String,
}

impl From<ListingIskanjedela> for Listing {
    fn from(iskanjedela_listing: ListingIskanjedela) -> Self {
        Self { 
            listing_id: iskanjedela_listing.id.clone(), 
            listing_title: format!("{} | {}", iskanjedela_listing.position_title, iskanjedela_listing.company_name), 
            listing_location: iskanjedela_listing.location, 
            listing_href: format!("https://www.iskanjedela.si/search-jobs?jobId={}", iskanjedela_listing.id) 
        }
    }
}