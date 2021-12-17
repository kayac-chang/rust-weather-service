use serde::{Deserialize, Serialize};

pub type Temperature = f32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    #[serde(rename = "lat")]
    pub latitude: f32,

    #[serde(rename = "lon")]
    pub longitude: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub city: String,
    pub town: String,
    pub name: String,

    #[serde(skip)]
    pub precipitation_per_day: f32,

    #[serde(skip)]
    pub altitude: f32,

    #[serde(rename = "temp")]
    pub temperature: Temperature,
    pub location: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub name: String,
    pub max_temperature: Temperature,
    pub min_temperature: Temperature,
    pub temperature_difference_per_day: Temperature,
}
