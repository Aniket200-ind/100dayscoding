use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct AirQualityIndex {
    pub list: Vec<AirQualityData>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AirQualityData {
    pub main: Main,
    pub components: Components,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Main {
    pub aqi: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Components {
    pub co: f32,
    pub no: f32,
    pub no2: f32,
    pub o3: f32,
    pub so2: f32,
    pub pm2_5: f32,
    pub pm10: f32,
    pub nh3: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum AirQuality {
    Good,
    Fair,
    Moderate,
    Poor,
    VeryPoor,
}
