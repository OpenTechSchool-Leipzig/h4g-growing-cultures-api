use anyhow::Result;
use growingcultures_server::{init_tracing, Application, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let settings = Settings {
        host: "0.0.0.0".into(),
        port: 3000,
    };
    let app = Application::new(settings)?;
    app.run_until_stopped().await
}
