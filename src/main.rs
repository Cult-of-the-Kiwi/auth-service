use auth_service::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("Starting auth service...");

    app::run().await?;

    Ok(())
}
