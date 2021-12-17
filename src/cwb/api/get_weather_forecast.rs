use super::super::logic;
use super::super::model::resp::Forecast;

use hyper::{http::Result, Body, Request, Response, StatusCode};
use serde::Serialize;

fn response<T>(data: T) -> Result<Response<Body>>
where
    T: Serialize,
{
    let payload =
        serde_json::to_string_pretty(&data).expect("error occured when serialize result payload");

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(payload))
}

pub async fn get_weather_forecast(_req: Request<Body>) -> Result<Response<Body>> {
    let data = logic::get_weather_forecast()
        .await
        .expect("error occured when get weather data");

    // find location
    let location = &data[0];
    let name = location.name.clone();
    let temperatures = &location.temperatures;

    // get max temperature
    let max = temperatures
        .into_iter()
        .max_by(|a, b| PartialOrd::partial_cmp(&a.1.max, &b.1.max).unwrap())
        .map(|(_, temperature)| temperature.max)
        .unwrap();

    // get min temperature
    let min = temperatures
        .into_iter()
        .min_by(|a, b| PartialOrd::partial_cmp(&a.1.min, &b.1.min).unwrap())
        .map(|(_, temperature)| temperature.min)
        .unwrap();

    // get difference per day
    let diff = temperatures
        .into_iter()
        .map(|(_, group)| group.max - group.min)
        .reduce(f32::max)
        .unwrap();

    response(Forecast {
        name,
        max_temperature: max,
        min_temperature: min,
        temperature_difference_per_day: diff,
    })
}
