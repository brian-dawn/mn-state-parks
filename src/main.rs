use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod render;

#[derive(Debug, Serialize, Deserialize)]
struct Cache {
    parks: Vec<api::ParsedPark>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check if a cache file exists.
    let sites = if let Ok(cache) = std::fs::read_to_string("cache.json") {
        let cache: Cache = serde_json::from_str(&cache)?;
        eprintln!("Using cache");
        cache.parks
    } else {
        let sites = api::fetch_all_campsites().await?;

        // Write out the cache.
        let cache = Cache {
            parks: sites.clone(),
        };
        let json = serde_json::to_string(&cache)?;
        std::fs::write("cache.json", json)?;

        sites
    };

    let html = render::render(&sites)?;
    println!("{}", html);

    Ok(())
}
