//! HTTP API, documented using OpenAPI.
//!
//! This crate contains everything that is exposed at the public API. This has
//! two consequences:
//!
//! 1. Changes to this crate usually lead to bumping the API version, with the
//! nature of the change giving a good idea if its a breaking change.
//!
//! 2. Changes to the API usually start in this crate, with the compiler
//! guiding you to the rest of the codebase.

mod modify;

use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::modify::ModifyInfo;

#[derive(OpenApi)]
#[openapi(
    paths(get_tour),
    components(schemas(Tour)),
    modifiers(&ModifyInfo)
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Tour {
    pub id: u32,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/tour/",
    tag = "tour",
    responses(
        (status = 200, body = Tour),
    )
)]
async fn get_tour(_id: Path<u32>) -> Result<Json<Tour>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

pub fn get_router() -> Router {
    Router::new().route("/tour/", get(get_tour))
}
