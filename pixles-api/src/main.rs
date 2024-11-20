use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    pixles_api_graphql::start().await?;
    Ok(())
}
