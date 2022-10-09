use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct ResponseIskanjedela {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub result_count: i32,
    pub total_count: i32,
    pub data: Vec<ListingIskanjedela>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingIskanjedela {
    pub id: String,
    pub position_title: String,
    pub company_name: String,
    pub location: String,
    pub active: bool,
}