use super::super::model::{cwb, resp, Error};
use crate::env;
use cwb::weather_data;
use rayon::prelude::*;
use reqwest::Url;

fn get_parameter_by(
    name: weather_data::ParameterName,
    list: &Vec<weather_data::Parameter>,
) -> Option<String> {
    list.iter()
        .find(|item| item.name == name)
        .map(|item| item.value.clone())
}

fn get_weather_data_by(
    name: weather_data::WeatherElementName,
    list: &Vec<weather_data::WeatherElement>,
) -> Option<String> {
    list.iter()
        .find(|item| item.name == name)
        .map(|item| item.value.clone())
}

/// -99 皆表示 該時刻因故無資料。
fn check_data_is_valid(data: f32) -> Result<f32, Error> {
    if data == -99.0 {
        Err("invalid data".into())
    } else {
        Ok(data)
    }
}

fn to_location(item: &weather_data::Record) -> Result<resp::Record, Error> {
    let name = item.name.clone();
    let latitude = item.lat.parse()?;
    let longitude = item.lon.parse()?;

    let city = get_parameter_by(weather_data::ParameterName::City, &item.parameters)
        .ok_or("city not found")?;

    let town = get_parameter_by(weather_data::ParameterName::Town, &item.parameters)
        .ok_or("town not found")?;

    let temperature = get_weather_data_by(
        weather_data::WeatherElementName::Temperature,
        &item.weather_elements,
    )
    .ok_or("temperature not found")?
    .parse()
    .map(check_data_is_valid)??;

    let altitude = get_weather_data_by(
        weather_data::WeatherElementName::Elevation,
        &item.weather_elements,
    )
    .ok_or("elevation not found")?
    .parse()
    .map(check_data_is_valid)??;

    let precipitation_per_day = get_weather_data_by(
        weather_data::WeatherElementName::PrecipitationPerDay,
        &item.weather_elements,
    )
    .ok_or("precipitation per day not found")?
    .parse()
    .map(check_data_is_valid)??;

    Ok(resp::Record {
        name,
        city,
        town,
        altitude,
        temperature,
        precipitation_per_day,
        location: resp::Position {
            latitude,
            longitude,
        },
    })
}

/// 取得全台測站即時資料
pub async fn get_weather_data() -> Result<Vec<resp::Record>, Error> {
    let api = env::get_cwb_api();
    let token = env::get_token();

    let url = Url::parse_with_params(
        &format!("{}/v1/rest/datastore/O-A0001-001", api),
        [("Authorization", token)],
    )?;

    // 打 API
    let res = reqwest::get(url.as_str()).await?;

    // 解析 API 資料 變成 json
    let data = res.json::<weather_data::Data>().await?;

    // 將 原本的 location 轉換成 指定回傳格式
    let locations: Vec<_> = data
        .records
        .locations
        .par_iter()
        .flat_map(to_location)
        .collect();

    Ok(locations)
}
