mod api;
pub mod logic;
mod model;

use hyper::{
    http::{Error, Result},
    Body, Request, Response, StatusCode,
};
use routerify::Router;

async fn not_found(_: Request<Body>) -> Result<Response<Body>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("NOT FOUND"))
}

pub fn service() -> Router<Body, Error> {
    Router::builder()
        .get("/weather", api::get_weather_data)
        .get("/forecast", api::get_weather_forecast)
        .any(not_found)
        .build()
        .unwrap()
}
