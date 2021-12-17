use std::cmp::Ordering;

use super::super::logic;
use super::super::model::resp::Record;

use hyper::{http::Result, Body, Request, Response, StatusCode};
use itertools::Itertools;
use querystring::querify;
use serde::Serialize;

fn sort_by_h24r(a: &Record, b: &Record) -> Ordering {
    PartialOrd::partial_cmp(&b.precipitation_per_day, &a.precipitation_per_day).unwrap()
}

fn sort_by_temp(a: &Record, b: &Record) -> Ordering {
    PartialOrd::partial_cmp(&a.temperature, &b.temperature).unwrap()
}

fn not_sort(_: &Record, _: &Record) -> Ordering {
    Ordering::Equal
}

fn response<T>(data: Vec<T>) -> Result<Response<Body>>
where
    T: Serialize,
{
    let payload =
        serde_json::to_string_pretty(&data).expect("error occured when serialize result payload");

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(payload))
}

fn group_by(value: &str, data: Vec<Record>) -> Result<Response<Body>> {
    let mut result = Vec::new();

    if value == "ELEV" {
        result = data
            .iter()
            .into_grouping_map_by(|item| match item.altitude as i32 {
                0..=500 => "0-500",
                501..=1000 => "500-1000",
                1001..=1500 => "1000-1500",
                1501..=2000 => "1500-2000",
                2001..=2500 => "2000-2500",
                2501..=3000 => "2500-3000",
                _ => "> 3000",
            })
            .min_by(|_, a, b| sort_by_temp(a, b))
            .into_iter()
            .collect();
    }

    response(result)
}

pub async fn get_weather_data(req: Request<Body>) -> Result<Response<Body>> {
    let mut data = logic::get_weather_data()
        .await
        .expect("error occured when get weather data");

    if let Some(queries) = req.uri().query() {
        // @TODO: change to pattern matching
        for (key, value) in querify(queries) {
            if key == "group_by" {
                return group_by(value, data);
            }

            if key == "order_by" {
                let cmp = match value {
                    "TEMP" => sort_by_temp,
                    "H_24R" => sort_by_h24r,
                    _ => not_sort,
                };

                data = data.into_iter().sorted_by(cmp).collect();
            }

            if key == "limit" {
                let value: usize = value
                    .parse()
                    .expect("error occured when parsing limit value");

                data = data.into_iter().take(value).collect();
            }
        }
    }

    response(data)
}
