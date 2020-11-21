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
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Close")]
    pub close: f64,
    #[serde(rename = "Volume")]
    pub volume: f64,
}
