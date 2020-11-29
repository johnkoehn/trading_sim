extern crate serde;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigError {
    path: String,
    message: String
}

impl ConfigError {
    pub fn new(message: String, path: String) -> ConfigError {
        ConfigError {
            message,
            path
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
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
    pub number_of_threads: u64,
    pub mutation_chance: f64,
    pub hamming: f64
}

impl Config {
    pub fn validate_config(&self) -> Vec<ConfigError> {
        let mut config_errors = Vec::<ConfigError>::new();

        if self.traits.number_of_averaging_periods.max < self.traits.number_of_averaging_periods.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.NumberOfAveragingPeriods.Max".to_string()));
        }

        if self.traits.minimum_buy_momentum.max < self.traits.minimum_buy_momentum.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.MinimumBuyMomentum.Max".to_string()));
        }

        if self.traits.maximum_buy_momentum.max < self.traits.maximum_buy_momentum.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.MaximumBuyMomentum.Max".to_string()));
        }

        if self.traits.trailing_stop_loss.max < self.traits.trailing_stop_loss.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.TrailingStopLoss.Max".to_string()));
        }

        // max will never be less then 0.0 because max cannot be less then min
        if self.traits.trailing_stop_loss.min < 0.0 {
            config_errors.push(ConfigError::new("Min cannot be less then 0".to_string(), "Traits.TrailingStopLoss.Min".to_string()));
        }

        if self.traits.stop_loss.max < self.traits.stop_loss.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.StopLoss.Max".to_string()));
        }

        if self.traits.stop_loss.min < 0.0 {
            config_errors.push(ConfigError::new("Min cannot be less then 0".to_string(), "Traits.StopLoss.Min".to_string()));
        }

        if self.traits.minimum_holding_periods.max < self.traits.minimum_holding_periods.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.MinimumHoldingPeriods.Max".to_string()));
        }

        if self.traits.maximum_holding_periods.max < self.traits.maximum_holding_periods.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.MaximumHoldingPeriods.Max".to_string()));
        }

        if self.traits.percent_purchase.max < self.traits.percent_purchase.min {
            config_errors.push(ConfigError::new("Max cannot be less then Min".to_string(), "Traits.PercentPurchase.Max".to_string()));
        }

        if self.traits.percent_purchase.min < 0.0 {
            config_errors.push(ConfigError::new("Min cannot be less then 0".to_string(), "Traits.PercentPurchase.Min".to_string()));
        }

        if self.traits.percent_purchase.max > 100.0 {
            config_errors.push(ConfigError::new("Max cannot be greater then 100".to_string(), "Traits.PercentPurchase.Max".to_string()));
        }

        if self.minimum_purchase_size < 0.0 {
            config_errors.push(ConfigError::new("Minimum Purchase Size cannot be less then 0".to_string(), "MinimumPurchaseSize".to_string()));
        }

        if self.starting_money < 0.0 {
            config_errors.push(ConfigError::new("Starting Money cannot be less then 0".to_string(), "StartingMoney".to_string()));
        }

        if self.transaction_fee_as_percentage > 1.0 || self.transaction_fee_as_percentage < 0.0 {
            config_errors.push(ConfigError::new("Transaction Fee As Percentage can only be from 0 to 1".to_string(), "TransactionFeeAsPercentage".to_string()));
        }

        if self.traits.target_sell_percentage.min <= 0.0 {
            config_errors.push(ConfigError::new("Min must be greater then 0".to_string(), "Traits.TargetedSellPrice.Min".to_string()));
        }

        if self.traits.target_sell_percentage.min > self.traits.target_sell_percentage.max {
            config_errors.push(ConfigError::new("Min must be less then Max".to_string(), "Traits.TargetSellPercentage.Min".to_string()));
        }

        if self.mutation_chance < 0.0 || self.mutation_chance > 1.0 {
            config_errors.push(ConfigError::new("Mutation Chance must be between 0 and 1".to_string(), "MutationChance".to_string()));
        }

        if self.hamming < 0.0 || self.hamming > 100.0 {
            config_errors.push(ConfigError::new("Hamming must be between 0 and 100".to_string(), "Hamming".to_string()));
        }

        return config_errors;
    }
}

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
        pub percent_purchase: PercentPurchase,
        pub target_sell_percentage: TargetSellPercentage
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TargetSellPercentage {
        pub min: f64,
        pub max: f64
    }
}
