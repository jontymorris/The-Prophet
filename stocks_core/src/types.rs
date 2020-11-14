use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stock {
    pub symbol: String,
    pub listing_date: String,
    pub history: Vec<Candle>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Candle {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Open")]
    pub open: f32,
    #[serde(rename = "High")]
    pub high: f32,
    #[serde(rename = "Low")]
    pub low: f32,
    #[serde(rename = "Close")]
    pub close: f32,
    #[serde(rename = "Volume")]
    pub volume: f32,
}

pub struct Bound {
    pub upper: f32,
    pub middle: f32,
    pub lower: f32,
}

pub struct Close {
    pub value: f32,
    pub percent_change: f32,
    pub date: String,
}
