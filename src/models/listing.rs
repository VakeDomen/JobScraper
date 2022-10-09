#[derive(Clone, Debug)]
pub struct Listing {
    pub listing_id: Option<String>,
    pub listing_title: Option<String>,
    pub listing_location: Option<String>,
    pub listing_href: Option<String>,
}
