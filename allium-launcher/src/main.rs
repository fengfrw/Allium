mod allium_launcher;
mod devices;
mod view;

use anyhow::Result;

use allium_launcher::AlliumLauncher;
use common::platform::{DefaultPlatform, Platform};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let platform = DefaultPlatform::new()?;
    let mut app = AlliumLauncher::new(platform)?;
    app.run_event_loop().await?;
    Ok(())
}
