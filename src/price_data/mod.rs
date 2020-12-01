extern crate serde;

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub time: u64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64
}