use anyhow::Result;

pub mod api;


#[tokio::main]
async fn main() -> Result<()> {

    let resp_place = api::fetch_place().await?;
    println!("{:#?}", resp_place);
    // let resp_parks = api::fetch_parks().await?;

    // println!("{:#?}", resp_parks);

    // let resp = api::fetch().await?;

    // println!("{:#?}", resp);

    Ok(())
}
