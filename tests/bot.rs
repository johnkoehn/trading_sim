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
            minimum_holding_periods: 1,
            maximum_holding_periods: 30,
            percent_purchase: 90.0,
            target_sell_percentage: 5.0
        }
    }

    fn generate_default_bot (traits: Traits) -> Bot {
        Bot {
            id: 0,
            traits,
            money: 1000.0,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new(),
            fitness: 0.0
        }
    }

    fn generate_price_history () -> Vec<PriceData> {
        let price_point1 = PriceData {
            time: 1515034800,
            low: 100.0,
            high: 110.0,
            open: 100.0,
            close: 102.0,
            volume: 100.0
        };
        let price_point2 = PriceData {
            time: 1515033900,
            low: 102.0,
            high: 105.0,
            open: 102.0,
            close: 105.0,
            volume: 100.0
        };

        let price_point3 = PriceData {
            time: 1515033000,
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
        config.validate_config();

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
        assert_eq!(first_sold_holding.sell_time, 1515033000);
        assert_eq!(first_sold_holding.purchase_time, 1515033900);
        assert_eq!(first_sold_holding.win, true);
        assert_eq!(first_sold_holding.sell_reason, SellReason::Forced);

        assert_relative_eq!(bot.money, 1057.5, max_relative = 0.001);
    }

    #[test]
    fn test_bot_sell_stop_loss() {
        let mut traits = generate_default_traits();
        traits.trailing_stop_loss = 5.0;

        let mut bot = generate_default_bot(traits);
        let mut price_history = generate_price_history();
        let config = generate_default_config();

        let mut price_point = price_history.get_mut(2).unwrap();
        price_point.close = 100.0;
        price_point.high = 105.0;

        let fourth_price_point = PriceData {
            time: 1515033000,
            low: 100.0,
            high: 110.0,
            open: 105.0,
            close: 110.0,
            volume: 100.0
        };
        price_history.push(fourth_price_point);

        let price_history_as_arc = Arc::from(price_history);

        bot.run_period(&price_history_as_arc, 0, &config);
        bot.run_period(&price_history_as_arc, 1, &config);
        bot.run_period(&price_history_as_arc, 2, &config);

        assert_eq!(bot.sold_holdings.len(), 1);
        assert_eq!(bot.current_holdings.len(), 0);

        let first_sold_holding = bot.sold_holdings.get(0).unwrap();
        assert_eq!(first_sold_holding.asset, Asset::ETH);
        assert_relative_eq!(first_sold_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.trailing_stop_loss, 99.75, max_relative = 0.001);
        assert_eq!(first_sold_holding.periods_held, 2);
        assert_relative_eq!(first_sold_holding.buy_fee, 6.3, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.sell_fee, 6.176, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_from_sell, 876.19, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.amount_gained, -30.116, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.percent_gained, -3.323, max_relative = 0.001);
        assert_eq!(first_sold_holding.win, false);
        assert_eq!(first_sold_holding.sell_reason, SellReason::StopLoss);

        assert_relative_eq!(bot.money, 969.9, max_relative = 0.001);
    }

    #[test]
    fn test_bot_sell_trailing_stop_loss() {
        let traits = generate_default_traits();
        let mut bot = generate_default_bot(traits);
        let mut price_history = generate_price_history();
        let config = generate_default_config();

        let mut price_point = price_history.get_mut(2).unwrap();
        price_point.close = 103.0;
        price_point.high = 105.0;

        let fourth_price_point = PriceData {
            time: 1515033000,
            low: 100.0,
            high: 110.0,
            open: 105.0,
            close: 110.0,
            volume: 100.0
        };
        price_history.push(fourth_price_point);

        let price_history_as_arc = Arc::from(price_history);

        bot.run_period(&price_history_as_arc, 0, &config);
        bot.run_period(&price_history_as_arc, 1, &config);
        bot.run_period(&price_history_as_arc, 2, &config);

        assert_eq!(bot.sold_holdings.len(), 1);
        assert_eq!(bot.current_holdings.len(), 0);

        let first_sold_holding = bot.sold_holdings.get(0).unwrap();
        assert_eq!(first_sold_holding.asset, Asset::ETH);
        assert_relative_eq!(first_sold_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.trailing_stop_loss, 103.95, max_relative = 0.001);
        assert_eq!(first_sold_holding.periods_held, 2);
        assert_relative_eq!(first_sold_holding.buy_fee, 6.3, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.sell_fee, 6.362, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_from_sell, 902.613, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.amount_gained, -3.838, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.percent_gained, -0.4235, max_relative = 0.001);
        assert_eq!(first_sold_holding.win, false);
        assert_eq!(first_sold_holding.sell_reason, SellReason::TrailingStopLoss);

        assert_relative_eq!(bot.money, 996.161, max_relative = 0.001);
    }

    #[test]
    fn test_bot_sell_max_periods_held() {
        let mut traits = generate_default_traits();

        traits.maximum_holding_periods = 3;

        let mut bot = generate_default_bot(traits);
        let mut price_history = generate_price_history();
        let config = generate_default_config();

        let mut price_point = price_history.get_mut(2).unwrap();
        price_point.close = 105.0;
        price_point.high = 105.0;

        let fourth_price_point = PriceData {
            time: 1515033000,
            low: 100.0,
            high: 106.0,
            open: 105.0,
            close: 105.0,
            volume: 100.0
        };
        price_history.push(fourth_price_point);
        let fifth_price_point = PriceData {
            time: 1515039000,
            low: 100.0,
            high: 106.0,
            open: 105.0,
            close: 105.0,
            volume: 100.0
        };
        price_history.push(fifth_price_point);

        let price_history_as_arc = Arc::from(price_history);

        bot.run_period(&price_history_as_arc, 0, &config);
        bot.run_period(&price_history_as_arc, 1, &config);
        bot.run_period(&price_history_as_arc, 2, &config);
        bot.run_period(&price_history_as_arc, 3, &config);

        assert_eq!(bot.sold_holdings.len(), 1);
        assert_eq!(bot.current_holdings.len(), 0);

        let first_sold_holding = bot.sold_holdings.get(0).unwrap();
        assert_eq!(first_sold_holding.asset, Asset::ETH);
        assert_relative_eq!(first_sold_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.trailing_stop_loss, 103.95, max_relative = 0.001);
        assert_eq!(first_sold_holding.periods_held, 3);
        assert_relative_eq!(first_sold_holding.buy_fee, 6.3, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.sell_fee, 6.485, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_from_sell, 919.982, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.amount_gained, 13.685, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.percent_gained, 1.510, max_relative = 0.001);
        assert_eq!(first_sold_holding.win, true);
        assert_eq!(first_sold_holding.sell_reason, SellReason::MaxPeriodsHeld);

        assert_relative_eq!(bot.money, 1013.6851, max_relative = 0.001);
    }

    #[test]
    fn test_bot_target_sell_percent() {
        let traits = generate_default_traits();
        let mut bot = generate_default_bot(traits);
        let mut price_history = generate_price_history();
        let config = generate_default_config();

        let mut price_point = price_history.get_mut(2).unwrap();
        price_point.close = 105.0;

        let fourth_price_point = PriceData {
            time: 1515033000,
            low: 100.0,
            high: 110.0,
            open: 105.0,
            close: 110.0,
            volume: 100.0
        };
        price_history.push(fourth_price_point);

        let price_history_as_arc = Arc::from(price_history);

        bot.run_period(&price_history_as_arc, 0, &config);
        bot.run_period(&price_history_as_arc, 1, &config);
        bot.run_period(&price_history_as_arc, 2, &config);

        assert_eq!(bot.sold_holdings.len(), 1);
        assert_eq!(bot.current_holdings.len(), 0);

        let first_sold_holding = bot.sold_holdings.get(0).unwrap();
        assert_eq!(first_sold_holding.asset, Asset::ETH);
        assert_relative_eq!(first_sold_holding.amount, 8.8235, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_spent, 906.29, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.purchase_price, 102.0, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.stop_loss, 100.98, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.trailing_stop_loss, 103.95, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.targeted_sell_price, 107.1, max_relative = 0.001);
        assert_eq!(first_sold_holding.periods_held, 2);
        assert_relative_eq!(first_sold_holding.buy_fee, 6.3, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.sell_fee, 6.6149, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.money_from_sell, 938.3819, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.amount_gained, 32.084, max_relative = 0.001);
        assert_relative_eq!(first_sold_holding.percent_gained, 3.540, max_relative = 0.001);
        assert_eq!(first_sold_holding.win, true);
        assert_eq!(first_sold_holding.sell_reason, SellReason::TargetedSellPrice);
        assert_relative_eq!(bot.money, 1032.084, max_relative = 0.001);
    }

    #[test]
    fn test_hamming_no_difference() {
        let traits = generate_default_traits();
        let bot_one = generate_default_bot(traits);
        let bot_two = generate_default_bot(traits);

        let hamming_value = bot_one.hamming(&bot_two);
        assert_relative_eq!(hamming_value, 0.0, max_relative = 0.0001);
    }

    #[test]
    fn test_hamming_difference() {
        let traits = generate_default_traits();
        let bot_one = generate_default_bot(traits);
        let mut bot_two = generate_default_bot(traits);

        bot_two.traits.maximum_buy_momentum = 4.0;
        bot_two.traits.number_of_averaging_periods = 10;

        let hamming_value = bot_one.hamming(&bot_two);
        assert_relative_eq!(hamming_value, 25.589, max_relative = 0.0001);
    }
}
