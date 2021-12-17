use std::{collections::HashMap, ops::Range};

use super::super::model::{cwb::forecast, resp::Temperature, Error};
use crate::env;

use chrono::{NaiveDate, NaiveDateTime, ParseResult};
use forecast::WeatherElementName;
use reqwest::Url;

pub type TimeRange = Range<NaiveDateTime>;
pub type TemperatureBetween = (TimeRange, Temperature);

#[derive(Debug, Default)]
pub struct TemperatureGroup {
    pub max: Temperature,
    pub min: Temperature,
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub temperatures: HashMap<NaiveDate, TemperatureGroup>,
}

impl Location {
    fn append_temperature(
        &mut self,
        name: &WeatherElementName,
        (range, temperature): TemperatureBetween,
    ) {
        let temperatures = &mut self.temperatures;

        for date in [range.start.date(), range.end.date()] {
            if !temperatures.contains_key(&date) {
                temperatures.insert(date.clone(), TemperatureGroup::default());
            }

            let group = match temperatures.get_mut(&date) {
                Some(group) => group,
                None => return,
            };

            match name {
                WeatherElementName::MinTemperature => {
                    if group.min != 0.0 {
                        group.min = f32::min(temperature, group.min);
                    } else {
                        group.min = temperature
                    }
                }
                WeatherElementName::MaxTemperature => {
                    group.max = f32::max(temperature, group.max);
                }
                _ => return,
            };
        }
    }
}

fn parse_time(time: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")
}

fn handle_temperature(item: forecast::Time) -> Result<TemperatureBetween, Error> {
    let start =
        parse_time(&item.start_time).expect("parsing error occured when serialize weather endtime");

    let end =
        parse_time(&item.end_time).expect("parsing error occured when serialize weather endtime");

    let temperature = item
        .value
        .get(0)
        .map(|item| item.value.parse())
        .expect("weather element value is empty")
        .expect("parsing error occured when serialize weather temperature");

    Ok((TimeRange { start, end }, temperature))
}

fn to_record(item: forecast::Location) -> Location {
    let mut location = Location {
        name: item.name,
        temperatures: HashMap::new(),
    };

    for element in item.weather_elements {
        match element.name {
            WeatherElementName::MinTemperature | WeatherElementName::MaxTemperature => {
                element
                    .time
                    .into_iter()
                    .flat_map(handle_temperature)
                    .for_each(|group| location.append_temperature(&element.name, group));
            }
            _ => (),
        };
    }

    location
}

/// 全台各鄉鎮市區預報
pub async fn get_weather_forecast() -> Result<Vec<Location>, Error> {
    let api = env::get_cwb_api();
    let token = env::get_token();

    let api = &format!("{}/v1/rest/datastore/F-D0047-093", api);
    let url = Url::parse_with_params(
        api,
        [
            ("Authorization", token),
            (
                "locationId",
                forecast::ForecastType::NewTaipeiCityInWeek.to_string(),
            ),
            ("limit", "1".to_owned()),
        ],
    )?;

    // 打 API
    let res = reqwest::get(url.as_str()).await?;

    // 解析 API 資料 變成 json
    let data = res.json::<forecast::Response>().await?;

    // 將 原本的 location 轉換成 指定回傳格式
    let locations = data
        .records
        .locations
        .into_iter()
        .flat_map(|wrapper| wrapper.location.into_iter().map(to_record))
        .collect();

    Ok(locations)
}
