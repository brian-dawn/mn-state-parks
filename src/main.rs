use anyhow::Result;

pub mod campsite;


#[tokio::main]
async fn main() -> Result<()> {

    let resp_parks = campsite::fetch_parks().await?;

    println!("{:#?}", resp_parks);

    let resp = campsite::fetch().await?;

    println!("{:#?}", resp);

    Ok(())
}
