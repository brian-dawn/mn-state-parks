use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    pub facility_id: String,
    pub unit_type_id: i32,
    pub start_date: String,
    pub in_season_only: bool,
    pub web_only: bool,

    #[serde(rename = "IsADA")]
    pub is_ada: bool,

    pub sleeping_unit_id: String,
    pub unit_types_group_ids: Vec<String>,
    pub unit_category_id: String,
    pub min_date: String, // MM/DD/YYYY
    pub max_date: String, // MM/DD/YYYY
    pub min_vehicle_length: u32

}



#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub facility: Facility
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Facility {

    pub units: HashMap<String, Unit>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unit {
    unit_id: i32,
    name: String,
    short_name: String,
    slices: HashMap<String, Slice>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Slice {
    date: String, // yyyy-mm-dd

    is_free: bool,
    min_stay: u32,

}



pub async fn fetch() -> Result<Response> {

    let json = Request {
        facility_id: "788".to_string(),
        unit_type_id: 0,
        start_date: "9-20-2022".to_string(),
        in_season_only: true,
        web_only: true,
        is_ada: false,
        unit_category_id: "25".to_string(),
        sleeping_unit_id: "31".to_string(),
        unit_types_group_ids: vec!["25".to_string()],
        min_date: "9/13/2022".to_string(),
        max_date: "1/11/2023".to_string(),
        min_vehicle_length: 0
    };



    let url = "https://mnrdr.usedirect.com/minnesotardr/rdr/search/grid";
    let resp = reqwest::Client::new()
    .post(url)
    .json(&json)
    .send()
    .await?
        .json::<Response>()
        .await?;

    Ok(resp)
}