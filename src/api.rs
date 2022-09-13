use std::{collections::HashMap, time::Duration, any::Any};

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};


pub enum UnitType {
    Backpacking = 177,
    BikeIn = 176,
    CartIn = 165
}
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
pub struct GridFacility {
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
    date: chrono::NaiveDate, // yyyy-mm-dd

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

pub async fn fetch_place(place_id: &str) -> Result<Place> {
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
        place_id: place_id.to_string(),
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

pub async fn fetch_facility(facility_id: &str) -> Result<GridFacility> {
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
        facility_id: facility_id.to_string(),
        unit_type_id: 0,
        start_date: "9-13-2022".to_string(),
        in_season_only: true,
        web_only: true,
        is_ada: false,
        unit_category_id: "25".to_string(),
        sleeping_unit_id: "31".to_string(),
        unit_types_group_ids: vec![],
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
        .json::<GridFacility>()
        .await?;

    Ok(resp)
}

pub async fn fetch_all_campsites() -> Result<Vec<GridFacility>> {
    //let parks = fetch_parks().await?;
    //for park in parks.iter() {
    //    println!("fetching {} (id={})", park.name, park.place_id);

        tokio::time::sleep(Duration::from_millis(100)).await;
        //let place = fetch_place(park.place_id.to_string().as_str()).await?;
        let place = fetch_place("70").await?;


        for facility in place.selected_place.facilities.values() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("\tfetching {}", facility.name);
            let grid = fetch_facility(facility.facility_id.to_string().as_str()).await?;

            for unit in grid.facility.units.values() {
                println!("\t\t{} - {}", unit.name, unit.short_name);

                let mut slices = unit.slices.values().collect::<Vec<_>>();
                slices.sort_by(|a, b| a.date.cmp(&b.date));

                for slice in slices {
                    println!("\t\t\t{} - {}", slice.date, if slice.is_free { "available" } else { "reserved" });
                }
            }
        }
    //}

    todo!("fetch all campsites")
}
