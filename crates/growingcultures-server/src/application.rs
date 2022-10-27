use std::net::TcpListener;
use std::num::NonZeroU16;

use anyhow::Result;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug)]
pub struct Settings {
    pub host: String,
    pub port: u16, // 0 für "zufälligen" Port
}

pub struct Application {
    host: String,
    port: NonZeroU16,
    listener: TcpListener,
}

impl Application {
    pub fn new(settings: Settings) -> Result<Self> {
        let address = format!("{}:{}", settings.host, settings.port);

        // `settings.port` can be `0`, requesting a random port from the
        // OS. Hence we need to bind first before we can know the actual port
        // our server will listen on.
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port().try_into()?;

        Ok(Self {
            host: settings.host,
            port,
            listener,
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> NonZeroU16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        let app = Router::new()
            .merge(growingcultures_api::get_router())
            .merge(
                SwaggerUi::new("/docs/*tail")
                    .url("/openapi.json", growingcultures_api::ApiDoc::openapi()),
            );

        axum::Server::from_tcp(self.listener)?
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
