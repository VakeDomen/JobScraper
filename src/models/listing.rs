#[derive(Clone, Debug)]
pub struct Sale {
    pub sale_id: Option<String>,
    pub sale_location: Option<String>,
    pub sale_href: Option<String>,
    pub sale_price: Option<String>,
    pub sale_size: Option<String>,
}
