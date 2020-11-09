extern crate serde;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub traits: traits::Traits
}

mod traits {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Traits {
        pub number_of_averaging_periods: NumberOfAveragingPeriods,
        pub buy_direction: BuyDirection,
        pub sell_direction: SellDirection,
        pub minimum_buy_momentum: MinimumBuyMomentum,
        pub maximum_buy_momentum: MaximumBuyMomentum,
        pub trailing_stop_loss: TrailingStopLoss,
        pub stop_loss: StopLoss,
        pub minimum_holding_periods: MinimumHoldingPeriods,
        pub maximum_holding_periods: MaximumHoldingPeriods
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NumberOfAveragingPeriods {
        pub min: u64,
        pub max: u64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BuyDirection {
        pub upward: bool,
        pub downward: bool
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SellDirection {
        pub upward: bool,
        pub downward: bool
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct MinimumBuyMomentum {
        pub min: f64,
        pub max: f64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct MaximumBuyMomentum {
        pub min: f64,
        pub max: f64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrailingStopLoss {
        pub min: f64,
        pub max: f64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StopLoss {
        pub min: f64,
        pub max: f64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct MinimumHoldingPeriods {
        pub min: u64,
        pub max: u64
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct MaximumHoldingPeriods {
        pub min: u64,
        pub max: u64
    }
}
