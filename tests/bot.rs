use trading_sim::bot::Bot;
use trading_sim::price_data::PriceData;
use trading_sim::bot::traits::Traits;
use trading_sim::bot::holdings::{CurrentHolding, SoldHolding, SellReason};
use trading_sim::config::Config;
use trading_sim::asset::Asset;
use std::sync::Arc;
use trading_sim::*;
use std::fs;
use std::f64;
extern crate trading_sim;
use chrono::{NaiveDateTime};

#[macro_use]
extern crate approx;



mod bot_assets {
    use super::*;
    // let default_traits: traits::Traits;

    fn generate_default_traits () -> Traits {
        Traits {
            number_of_averaging_periods: 1,
            minimum_buy_momentum: 1.0,
            maximum_buy_momentum: 2.0,
            trailing_stop_loss: 1.0,
            stop_loss: 1.0,
            minimum_holding_periods: 10,
            maximum_holding_periods: 30,
            percent_purchase: 90.0
        }
    }

    fn generate_default_bot (traits: Traits) -> Bot {
        Bot {
            traits,
            money: 1000.0,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new()
        }
    }

    fn generate_price_history () -> Vec<PriceData> {
        let price_point1 = PriceData {
            time: NaiveDateTime::from_timestamp(1515034800, 0),
            low: 100.0,
            high: 110.0,
            open: 100.0,
            close: 102.0,
            volume: 100.0
        };
        let price_point2 = PriceData {
            time: NaiveDateTime::from_timestamp(1515033900, 0),
            low: 102.0,
            high: 105.0,
            open: 102.0,
            close: 105.0,
            volume: 100.0
        };

        let price_point3 = PriceData {
            time: NaiveDateTime::from_timestamp(1515033000, 0),
            low: 100.0,
            high: 110.0,
            open: 105.0,
            close: 110.0,
            volume: 100.0
        };

        vec!(price_point1, price_point2, price_point3)
    }

    fn generate_default_config () -> Arc<Config> {
        let config_as_yaml = fs::read_to_string("./tests/test_data/example_config.yaml").unwrap();
        let config: Config = serde_yaml::from_str(&config_as_yaml.as_str()).unwrap();
        config.validate_config().unwrap();

        Arc::new(config)
    }

    #[test]
    fn test_bot_simple_cycle() {
        let traits = generate_default_traits();
        let mut bot = generate_default_bot(traits);
        let price_history = generate_price_history();
        let config = generate_default_config();

        let price_history_as_arc = Arc::from(price_history);

        bot.run_period(&price_history_as_arc, 0, &config);

        assert_eq!(bot.current_holdings.len(), 0);

        bot.run_period(&price_history_as_arc, 1, &config);
        assert_eq!(bot.current_holdings.len(), 1);

        let first_holding = bot.current_holdings.get(0).unwrap();
        assert_eq!(first_holding.asset, Asset::ETH);
        assert_relative_eq!(first_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_holding.trailing_stop_loss, 103.95, max_relative = 0.001);
        assert_eq!(first_holding.periods_held, 1);
        assert_relative_eq!(first_holding.buy_fee, 6.29, max_relative = 0.01);

        bot.run_period(&price_history_as_arc, 2, &config);
        assert_eq!(bot.sold_holdings.len(), 1);
        assert_eq!(bot.current_holdings.len(), 0);

        let first_sold_holding = bot.sold_holdings.get(0).unwrap();
        println!("{:?}", first_sold_holding);
        assert_eq!(first_sold_holding.asset, Asset::ETH);
        assert_relative_eq!(first_sold_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.trailing_stop_loss, 108.9, max_relative = 0.001);
        assert_eq!(first_sold_holding.periods_held, 2);
        assert_relative_eq!(first_sold_holding.buy_fee, 6.3, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.sell_fee, 6.79, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_from_sell, 963.79, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.amount_gained, 57.5, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.percent_gained, 6.34, max_relative = 0.001);
        assert_eq!(first_sold_holding.win, true);
        assert_eq!(first_sold_holding.sell_reason, SellReason::Forced);

        assert_relative_eq!(bot.money, 1057.5, max_relative = 0.001);
    }

    // #[test]
    // fn test_bot_sell_stop_loss() {
    //     let mut traits = generate_default_traits();
    //     traits.trailing_stop_loss = 5.0;

    //     let mut bot = generate_default_bot(traits);
    //     let mut price_history = generate_price_history();
    //     let config = generate_default_config();

    //     let mut price_point = price_history.get_mut(2).unwrap();
    //     price_point.close = 100.0;

    //     let fourth_price_point = PriceData {
    //         time: NaiveDateTime::from_timestamp(1515033000, 0),
    //         low: 100.0,
    //         high: 110.0,
    //         open: 105.0,
    //         close: 110.0,
    //         volume: 100.0
    //     };
    //     price_history.push(fourth_price_point);

    //     let price_history_as_arc = Arc::from(price_history);

    //     bot.run_period(&price_history_as_arc, 0, &config);
    //     bot.run_period(&price_history_as_arc, 1, &config);
    //     bot.run_period(&price_history_as_arc, 2, &config);

    //     assert_eq!(bot.sold_holdings.len(), 1);
    //     assert_eq!(bot.current_holdings.len(), 0);

    //     let first_sold_holding = bot.sold_holdings.get(0).unwrap();
    //     assert_eq!(first_sold_holding.asset, Asset::ETH);
    //     ulps_eq!(first_sold_holding.amount, 8.76);
    //     ulps_eq!(first_sold_holding.money_spent, 900.0);
    //     ulps_eq!(first_sold_holding.purchase_price, 102.0);
    //     ulps_eq!(first_sold_holding.stop_loss, 100.98);
    //     ulps_eq!(first_sold_holding.trailing_stop_loss, 96.9);
    //     assert_eq!(first_sold_holding.periods_held, 2);
    //     ulps_eq!(first_sold_holding.buy_fee, 0.714);
    //     ulps_eq!(first_sold_holding.sell_fee, 6.132);
    //     ulps_eq!(first_sold_holding.money_from_sell, 956.855);
    //     ulps_eq!(first_sold_holding.amount_gained, 56.855);
    //     ulps_eq!(first_sold_holding.percent_gained, 6.31);
    //     assert_eq!(first_sold_holding.win, false);
    //     assert_eq!(first_sold_holding.sell_reason, SellReason::StopLoss);
    // }

    // #[test]
    // fn test_bot_sell_trailing_stop_loss() {
    //     let traits = generate_default_traits();
    //     let mut bot = generate_default_bot(traits);
    //     let price_history = generate_price_history();
    //     let config = generate_default_config();

    //     let price_history_as_arc = Arc::from(price_history);

    //     bot.run_period(&price_history_as_arc, 0, &config)
    // }

    // #[test]
    // fn test_bot_sell_max_periods_held() {
    //     let mut traits = generate_default_traits();
    //     traits.trailing_stop_loss = 2.0;

    //     let mut bot = generate_default_bot(traits);
    //     let price_history = generate_price_history();
    //     let config = generate_default_config();

    //     let price_history_as_arc = Arc::from(price_history);

    //     bot.run_period(&price_history_as_arc, 0, &config)
    // }
}
