use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EntityType {
    City,
    Park,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatePark {
    pub city_park_id: i32,
    pub enterprise_id: i32,
    pub entity_type: Option<EntityType>,
    pub is_active: bool,
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub place_id: u32,
    pub park_size: String, // NOTE: this should be an enum but I got mad that there's case insensitivity happening. small, Small, etc.
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Place {
    pub selected_place: SelectedPlace,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SelectedPlace {
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub available: bool,
    pub facilities: HashMap<i32, PlaceFacility>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceFacility {
    pub facility_id: i32,
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub available: bool,
    pub in_season: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub facility: Facility,
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

pub async fn fetch_parks() -> Result<Vec<StatePark>> {
    let url = "https://mnrdr.usedirect.com/minnesotardr/rdr/fd/citypark";
    let resp = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, StatePark>>()
        .await?;

    Ok(resp
        .values()
        // Only show parks, not cities.
        .filter(|park| park.entity_type == Some(EntityType::Park))
        .cloned()
        .collect())
}

pub async fn fetch_place() -> Result<Place> {
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct Grid {
        place_id: String,

        latitude: f64,
        longitude: f64,
        highlighted_place_id: i32,
        start_date: String,
        nights: String,
        count_nearby: bool,
        nearby_limit: i32,
        nearby_only_available: bool,
        nearby_count_limit: i32,
        sort: String,
        customer_id: String,
        refresh_favourites: bool,
        #[serde(rename = "IsADA")]
        is_ada: bool,
        unit_category_id: String,
        sleeping_unit_id: String,
        min_vehicle_length: i32,
        unit_types_group_ids: Vec<String>,
        highlights: Vec<String>,
        amenity_ids: Vec<String>,
    }

    let json = Grid {
        place_id: "117".to_string(),
        latitude: 0.0,
        longitude: 0.0,
        highlighted_place_id: 0,
        start_date: "09-15-2022".to_string(),
        nights: "1".to_string(),
        count_nearby: true,
        nearby_limit: 100,
        nearby_only_available: false,
        nearby_count_limit: 10,
        sort: "Distance".to_string(),
        customer_id: "0".to_string(),
        refresh_favourites: true,
        is_ada: false,
        unit_category_id: "25".to_string(),
        sleeping_unit_id: "31".to_string(),
        min_vehicle_length: 0,
        unit_types_group_ids: vec![],
        highlights: vec![],
        amenity_ids: vec![],
    };

    let url = "https://mnrdr.usedirect.com/minnesotardr/rdr/search/place";

    let resp = reqwest::Client::new().post(url).json(&json).send().await?;

    let resp_body = resp.json::<Place>().await?;

    Ok(resp_body)
    // From a place we get facilities and that's how we fetch campsites.
}

pub async fn fetch() -> Result<Response> {
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
        pub min_vehicle_length: u32,
    }

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
        min_vehicle_length: 0,
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

pub async fn fetch_all_campsites() -> Result<Vec<Campsite>> {
    let mut campsites = vec![];

    let places = fetch_places().await?;
    for place in places {
        let facilities = fetch_facilities(place.place_id).await?;
        for facility in facilities {
            let campsites = fetch_campsites(facility.facility_id).await?;
            for campsite in campsites {
                campsites.push(campsite);
            }
        }
    }

    Ok(campsites)
}
