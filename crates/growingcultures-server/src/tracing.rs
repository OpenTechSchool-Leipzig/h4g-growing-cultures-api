use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer();
    let env_filter = EnvFilter::from_env("GROWINGCULTURES_LOG");

    tracing_subscriber::registry()
        .with(stdout_log)
        .with(env_filter)
        .init();
}
