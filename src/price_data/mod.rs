extern crate serde;
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDataRaw {
    pub time: i64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug)]
pub struct PriceData {
    pub time: NaiveDateTime,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64
}

impl PriceData {
    pub fn new(price_data_raw: &PriceDataRaw) ->  PriceData {
        PriceData {
            time: NaiveDateTime::from_timestamp(price_data_raw.time, 0),
            low: price_data_raw.low,
            high: price_data_raw.high,
            open: price_data_raw.open,
            close: price_data_raw.close,
            volume: price_data_raw.volume
        }
    }
}
