use color_eyre::Result;

use calculator_service::app_run;
use calculator_service::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = AppConfig::from_env()?;
    app_run(conf).await?.await?;

    Ok(())
}
