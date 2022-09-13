use std::collections::HashMap;

use serde::{Deserialize, Serialize, Deserializer};
use anyhow::Result;


#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EntityType {
    City,
    Park
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatePark {
pub  city_park_id: i32,
pub  enterprise_id: i32,
pub  entity_type: Option<EntityType>,
pub  is_active: bool,
pub  latitude: f64,
pub  longitude: f64,
pub  name: String,
 pub  place_id: u32,
 pub park_size: String // NOTE: this should be an enum but I got mad that there's case insensitivity happening. small, Small, etc.

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
    pub facilities: HashMap<i32, PlaceFacility>
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


pub async fn fetch_parks() -> Result<Vec<StatePark>> {


    let url = "https://mnrdr.usedirect.com/minnesotardr/rdr/fd/citypark";
    let resp = reqwest::Client::new()
    .get(url)
    .send()
    .await?
        .json::<HashMap<String, StatePark>>()
        .await?;

        Ok(resp.values()
        // Only show parks, not cities.
        .filter(|park| park.entity_type == Some(EntityType::Park))
        .cloned().collect())
}


pub async fn fetch_place() -> Result<Place>{

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
    struct Request {
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
        #[serde(rename="IsADA")]
        is_ada: bool,
        unit_category_id: String,
        sleeping_unit_id: String,
        min_vehicle_length: i32,
        unit_types_group_ids: Vec<String>,
        highlights: Vec<String>,
        amenity_ids: Vec<String>,

    }

    /*
{"PlaceId":"117","Latitude":0,"Longitude":0,"HighlightedPlaceId":0,"StartDate":"09-15-2022","Nights":"1","CountNearby":true,
"NearbyLimit":100,"NearbyOnlyAvailable":false,"NearbyCountLimit":10,"Sort":"Distance","CustomerId":"0",
"RefreshFavourites":true,"IsADA":false,"UnitCategoryId":"25","SleepingUnitId":"31","MinVehicleLength":0,
"UnitTypesGroupIds":[],"Highlights":[],"AmenityIds":[]}
     */


     /*response


{"Message":"Built in 9.0203 ms size 13918 bytes on MNDNR-RDR1","SelectedPlaceId":117,"HighlightedPlaceId":0,
"Latitude":0.0,"Longitude":0.0,"StartDate":"2022-09-15","EndDate":"2022-09-15","NightsRequested":1,
"NightsActual":1,"CountNearby":true,"NearbyLimit":100,"Sort":"Distance","CustomerId":null,
"Filters":{"IsADA":"False","UnitCategoryId":"25","SleepingUnitId":"31","MinVehicleLength":"0"},"AvailablePlaces":11,
"SelectedPlace":{"PlaceId":117,"Name":"George H. Crosby Manitou State Park",
"Description":"Waterfalls cascade through a volcanic canyon at this north-country wilderness park, where rugged trails lead through old-growth forests to spectacular views and secluded campsites along the Manitou River. Watch for moose, deer, bear, and wolves.","HasAlerts":false,
"IsFavourite":false,"Allhighlights":"Hiking trailsCarry-in boat access","Url":"http://www.dnr.state.mn.us/state_parks/george_crosby_manitou/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small",
"Latitude":47.47946031,"Longitude":-91.12301865,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":0,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,
"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-13T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},
"Facilities":{"788":{"FacilityId":788,
"Name":"Backpack Campsites","Description":"Campground is open year-round<br>Showers are not provided in this campground<br><b>Arrive prepared.</b> Go to mndnr.gov/crosbymanitou for visitor alerts and seasonal updates.","RateMessage":null,"FacilityType":2,"FacilityTypeNew":1,"InSeason":true,"Available":true,"AvailableFiltered":true,
"Restrictions":{"FutureBookingStarts":"2022-09-13T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Latitude":47.47861679,"Longitude":-91.09667395,"Category":"Campgrounds","EnableCheckOccupancy":false,"AvailableOccupancy":null,"FacilityAllowWebBooking":true,"UnitTypes":{"177":{"UnitTypeId":177,"UseType":4,
"Name":"Backpack","Available":true,"AvailableFiltered":true,"UnitCategoryId":25,"UnitTypeGroupId":177,"MaxVehicleLength":0,"HasAda":false,"Restrictions":null,"AvailableCount":19}},"IsAvailableForGroup":false,"IsAvailableForPatron":false,"IsAvailableForEducationalGorup":false,"IsAvailableForCto":false,"FacilityBehaviourType":0}},"IsAvailableForGreatwalk":false},"NearbyPlaces":[{"PlaceId":104,"Name":"Tettegouche State Park","Description":"A spectacular example of the North Shore’s natural beauty, this park has scenic overlooks galore. Hikers and skiers can view the Sawtooth Mountains and several waterfalls along the Baptism River, including High Falls, the highest waterfall entirely inside Minnesota’s border.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programsHiking trailsGroomed classic x-country ski<BR>Snowmobile Trails <BR>Visitor center<BR>Picnic shelter<BR>Fishing pierCarry-in boat access<BR>Winter warming shelter<BR>Canoe Rental","Url":"http://www.dnr.state.mn.us/state_parks/tettegouche/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"medium","Latitude":47.33988342,"Longitude":-91.19635362,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":11,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":103,"Name":"Temperance River State Park","Description":"Waterfalls, deep potholes, and eye-catching geologic features are everywhere along the deep, narrow Temperance River gorge. Take the Superior Hiking Trail through the gorge to Carlton Peak (el. 1,526 ft.) or fish one of the park’s designated trout streams.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programsHiking trails<BR>Paved bike trailsSnowmobile Trails <BR>Carry-in boat access","Url":"http://www.dnr.state.mn.us/state_parks/temperance_river/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.55267123,"Longitude":-90.87747661,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":13,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":70,"Name":"Split Rock Lighthouse State Park","Description":"You know the lighthouse, but have you seen the waterfalls along Split Rock River, skipped stones at Pebble Beach, or hiked and skied this park’s beautiful trails? Bring bikes and ride the Gitchi-Gami State Trail to Gooseberry Falls State Park.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programsHiking trails<BR>Paved bike trails<BR>Visitor center<BR>Picnic shelter<BR>Fishing pier<BR>Winter warming shelter","Url":"http://www.dnr.state.mn.us/state_parks/split_rock_lighthouse/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.20565857,"Longitude":-91.36825529,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":23,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":118,"Name":"Gooseberry Falls State Park","Description":"This park is the gateway to the North Shore, known for its spectacular waterfalls and Northwoods wildlife. Bring bikes and ride the Gitchi-Gami State Trail to Split Rock Lighthouse State Park.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programs Hiking trails<BR>Paved bike trails<BR>Groomed classic x-country ski<BR>Snowmobile Trails <BR>Visitor center<BR>Picnic shelter Winter warming shelter","Url":"http://www.dnr.state.mn.us/state_parks/gooseberry_falls/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"medium","Latitude":47.1398681,"Longitude":-91.47335803,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":29,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":68,"Name":"Cascade River State Park","Description":"Wildlife and waterfalls will thrill visitors to this scenic spot in the Sawtooth Mountains. Hike to the top of Moose Mountain (el. 1,148 ft.) and Lookout Mountain (el. 1,200 ft.).","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Hiking trails Groomed classic x-country ski<BR>Snowmobile Trails Picnic shelter Winter warming shelter","Url":"http://www.dnr.state.mn.us/state_parks/cascade_river/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.71096785,"Longitude":-90.50592618,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":33,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":136,"Name":"Bear Head Lake State Park","Description":"Voted America’s Favorite Park in 2010, it offers excellent canoeing, fishing, and camping in a Northwoods setting similar to the nearby Boundary Waters Canoe Area.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Hiking trailsMountain Bike Trails<BR>Horse Trails<BR>Groomed classic x-country ski<BR>Snowmobile Trails Picnic shelter<BR>Swimming beachFishing pier<BR>Drive-in boat access<BR>Carry-in boat access<BR>Winter warming shelter<BR>Canoe Rental<BR>Kayak Rental <BR>Stand-up Paddleboard Rental<BR>Boat Rental","Url":"http://www.dnr.state.mn.us/state_parks/bear_head_lake/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.8089677,"Longitude":-92.0628353,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":50,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":122,"Name":"Judge C. R. Magney State Park","Description":"Enjoy trout fishing and spectacular scenery as you hike along the Brule River to the famous Devil’s Kettle waterfall, where half of the river plunges 50 feet into a pool, and the rest disappears.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Hiking trails","Url":"http://www.dnr.state.mn.us/state_parks/judge_cr_magney/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.81742324,"Longitude":-90.05375405,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":55,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":102,"Name":"Lake Vermilion-Soudan Underground Mine State Park","Description":"Journey a half-mile underground to explore the world of mining or tour a fascinating physics lab. This park preserves five miles of shoreline and public access to a 40,000-acre lake with 368 islands. Hike the 2.4-mile Alaska Shaft trail for an overview.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programsHiking trails<BR>Paved bike trailsSnowmobile Trails <BR>Visitor center<BR>Picnic shelter<BR>Fishing pier","Url":"http://www.dnr.state.mn.us/state_parks/lake_vermilion_soudan/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.84386,"Longitude":-92.19225,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":56,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":121,"Name":"Jay Cooke State Park","Description":"Best known for its swinging bridge, which leads across the thundering St. Louis River, this park features outstanding trails for bikers (both mountain and tour), hikers, horseback riders, and skiers. Park trails link to the Willard Munger State Trail.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Nature programs Hiking trails<BR>Paved bike trails<BR>Mountain Bike Trails<BR>Horse Trails<BR>Groomed classic x-country ski<BR>Snowmobile Trails <BR>Visitor center<BR>Picnic shelter Winter warming shelter","Url":"http://www.dnr.state.mn.us/state_parks/jay_cooke/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":46.6616037,"Longitude":-92.39908935,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":83,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false},{"PlaceId":88,"Name":"McCarthy Beach State Park","Description":"Relax at a nationally recognized beach, hike and ski scenic trails within the park, or go snowmobiling, mountain biking, and horseback riding on the Taconite State Trail.","HasAlerts":false,"IsFavourite":false,"Allhighlights":"Hiking trailsMountain Bike Trails<BR>Horse TrailsSnowmobile Trails Picnic shelter<BR>Swimming beachFishing pier<BR>Drive-in boat access<BR>Carry-in boat accessCanoe Rental<BR>Kayak Rental <BR>Stand-up Paddleboard Rental<BR>Boat Rental","Url":"http://www.dnr.state.mn.us/state_parks/mccarthy_beach/index.html","ImageUrl":null,"BannerUrl":null,"ParkSize":"Small","Latitude":47.66890792,"Longitude":-93.03154944,"TimeZone":"Central Standard Time","TimeStamp":"2022-09-13 11:48:03","MilesFromSelected":90,"Available":true,"AvailableFiltered":true,"ParkCategoryId":1,"ParkActivity":1,"ParkPopularity":0,"AvailableUnitCount":0,"Restrictions":{"FutureBookingStarts":"2022-09-14T00:00:00-05:00","FutureBookingEnds":"2023-01-11T00:00:00-06:00","MinimumStay":1,"MaximumStay":14,"IsRestrictionValid":true},"Facilities":{},"IsAvailableForGreatwalk":false}]}
     
     
     */

     let json = Request {
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

    let resp = reqwest::Client::new()
    .post(url)
    .json(&json)
    .send()
    .await?;
        
    let resp_body = resp.json::<Place>()
        .await?;

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
    pub min_vehicle_length: u32

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