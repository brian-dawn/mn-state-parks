use anyhow::Result;

pub mod api;

#[tokio::main]
async fn main() -> Result<()> {
    api::fetch_all_campsites().await?;
    // let resp_parks = api::fetch_parks().await?;

    // println!("{:#?}", resp_parks);

    // let resp = api::fetch().await?;

    // println!("{:#?}", resp);

    Ok(())
}
