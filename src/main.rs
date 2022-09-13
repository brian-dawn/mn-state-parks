use anyhow::Result;

pub mod campsite;


#[tokio::main]
async fn main() -> Result<()> {

    let resp = campsite::fetch().await?;

    println!("{:#?}", resp);

    Ok(())
}
