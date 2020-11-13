extern crate serde;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ConfigError {
    description: String
}

impl ConfigError {
    pub fn new(description: String) -> ConfigError {
        ConfigError {
            description
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ConfigError {

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub traits: traits::Traits,
    pub number_of_bots: u64,
    pub number_of_generations: u64,
    pub starting_money: f64,
    pub minimum_purchase_size: f64,
    pub transaction_fee_as_percentage: f64,
    pub number_of_threads: u64
}

impl Config {
    pub fn validate_config(&self) -> Result<(), ConfigError> {

        if self.traits.number_of_averaging_periods.max < self.traits.number_of_averaging_periods.min {
            return Err(ConfigError::new("Traits.NumberOfAveragingPeriod.Max cannot be less then Traits.NumberOfAveragingPeriod.Min".to_string()))
        }

        if self.traits.minimum_buy_momentum.max < self.traits.minimum_buy_momentum.min {
            return Err(ConfigError::new("Traits.MinimumBuyMomentum.Max cannot be less then Traits.MinimumBuyMomentum.Min".to_string()))
        }

        if self.traits.maximum_buy_momentum.max < self.traits.maximum_buy_momentum.min {
            return Err(ConfigError::new("Traits.MaximumBuyMomentum.Max cannot be less then Traits.MaximumBuyMomentum.Min".to_string()))
        }

        if self.traits.trailing_stop_loss.max < self.traits.trailing_stop_loss.min {
            return Err(ConfigError::new("Traits.TrailingStopLoss.Max cannot be less then Traits.TrailingStopLoss.Min".to_string()))
        }

        // max will never be less then 0.0 because max cannot be less then min
        if self.traits.trailing_stop_loss.min < 0.0 {
            return Err(ConfigError::new("Traits.TrailingStopLoss.Min cannot be less then 0".to_string()))
        }

        if self.traits.stop_loss.max < self.traits.stop_loss.min {
            return Err(ConfigError::new("Traits.StopLoss.Max cannot be less then Traits.StopLoss.Min".to_string()))
        }

        if self.traits.stop_loss.min < 0.0 {
            return Err(ConfigError::new("Traits.StopLoss.Min cannot be less then 0".to_string()))
        }

        if self.traits.minimum_holding_periods.max < self.traits.minimum_holding_periods.min {
            return Err(ConfigError::new("Traits.MinimumHoldingPeriods.Max cannot be less then Traits.MinimumHoldingPeriods.Min".to_string()))
        }

        if self.traits.maximum_holding_periods.max < self.traits.maximum_holding_periods.min {
            return Err(ConfigError::new("Traits.MaximumHoldingPeriods.Max cannot be less then Traits.MaximumHoldingPeriods.Min".to_string()))
        }

        if self.traits.percent_purchase.max < self.traits.percent_purchase.min {
            return Err(ConfigError::new("Traits.PercentPurchase.Max cannot be less then Traits.PercentPurchase.Min".to_string()))
        }

        if self.traits.percent_purchase.max < self.traits.percent_purchase.min {
            return Err(ConfigError::new("Traits.PercentPurchase.Max cannot be less then Traits.PercentPurchase.Min".to_string()))
        }

        if self.traits.percent_purchase.min < 0.0 {
            return Err(ConfigError::new("Traits.PercentPurchase.Min cannot be less then 0".to_string()))
        }

        if self.traits.percent_purchase.max > 100.0 {
            return Err(ConfigError::new("Traits.PercentPurchase.Max cannot be greater then 100".to_string()))
        }

        if self.minimum_purchase_size < 0.0 {
            return Err(ConfigError::new("MinimumPurchaseSize cannot be less then 0".to_string()))

        }

        if self.starting_money < 0.0 {
            return Err(ConfigError::new("StartingMoney cannot be less then 0".to_string()))

        }

        if self.transaction_fee_as_percentage > 1.0 || self.transaction_fee_as_percentage < 0.0 {
            return Err(ConfigError::new("TransactionFeeAsPercentage can only be from 0 to 1".to_string()))
        }

        Ok(())
    }
}

// Add type to each ????
mod traits {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Traits {
        pub number_of_averaging_periods: NumberOfAveragingPeriods,
        pub minimum_buy_momentum: MinimumBuyMomentum,
        pub maximum_buy_momentum: MaximumBuyMomentum,
        pub trailing_stop_loss: TrailingStopLoss,
        pub stop_loss: StopLoss,
        pub minimum_holding_periods: MinimumHoldingPeriods,
        pub maximum_holding_periods: MaximumHoldingPeriods,
        pub percent_purchase: PercentPurchase
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NumberOfAveragingPeriods {
        pub min: u64,
        pub max: u64
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct PercentPurchase {
        pub min: f64,
        pub max: f64
    }
}
