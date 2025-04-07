mod proxy;
mod config;

use proxy::Proxy;
use tracing_subscriber::fmt::Subscriber;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Subscriber::builder().init();

    let config = Config::new(vec![
        "http://localhost:8081".to_string(),
        "http://localhost:8082".to_string(),
    ]);

    let proxy = Proxy::new(config);

    proxy.run(([0, 0, 0, 0], 8080)).await?;

    Ok(())
}
