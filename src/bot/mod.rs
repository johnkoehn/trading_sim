pub mod traits;
pub mod holdings;
use traits::Traits;
use holdings::*;
use std::sync::Arc;
use rand::Rng;
use crate::config::Config;
use crate::asset::Asset;
use crate::price_data::PriceData;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub id: u64,
    pub traits: Traits,
    pub money: f64,
    pub current_holdings: Vec<CurrentHolding>,
    pub sold_holdings: Vec<SoldHolding>,
    pub fitness: f64,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>
}

fn calculate_momentum(current_price: f64, previous_price: f64) -> f64 {
    ((current_price - previous_price) / previous_price) * 100.0
}

fn get_sell_reason(traits: &Traits, holding: &CurrentHolding, current_price_data: &PriceData) -> SellReason {
    // targeted sell price must always execute before anything else baby
    if holding.targeted_sell_price < current_price_data.high {
        return SellReason::TargetedSellPrice;
    }

    if holding.periods_held >= traits.maximum_holding_periods {
        return SellReason::MaxPeriodsHeld;
    }

    if holding.periods_held < traits.minimum_holding_periods {
        return SellReason::None;
    }

    if current_price_data.close <= holding.stop_loss {
        return SellReason::StopLoss
    }

    if current_price_data.close <= holding.trailing_stop_loss {
        return SellReason::TrailingStopLoss
    }

    SellReason::None
}

fn calculate_sell_fee(price: f64, transaction_fee_as_percentage: f64, amount: f64) -> f64 {
    price * amount * transaction_fee_as_percentage
}

fn calculate_money_from_sell(price: f64, sell_fee: f64, amount: f64) -> f64 {
    (price * amount) - sell_fee
}

fn calculate_amount_to_buy(money_to_spend: f64, current_price: f64) -> f64 {
    // we round to four decimal places
    let purchase_amount = money_to_spend /  current_price;
    return (purchase_amount * 10000.0).round() / 10000.0
}

impl Bot {
    pub fn new(config: &Config, id: u64) -> Bot {
        Bot {
            id,
            traits: Traits::new(config),
            money: config.starting_money,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new(),
            fitness: 0.0,
            start_time: None,
            end_time: None
        }
    }

    pub fn create_clone(&self, config: &Config, id: u64) -> Bot {
        let traits_copy = self.traits.clone();
        let money = config.starting_money;

        Bot {
            id,
            traits: traits_copy,
            money,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new(),
            fitness: 0.0,
            start_time: None,
            end_time: None
        }
    }

    fn handle_buy(&mut self, config: &Config, current_price_data: &PriceData, old_price_data: &PriceData) {
        let money_to_spend = self.money * (self.traits.percent_purchase / 100.0);
        if money_to_spend < config.minimum_purchase_size {
            return;
        }

        let current_price = current_price_data.open;
        let momentum = calculate_momentum(current_price, old_price_data.open);

        if momentum < self.traits.minimum_buy_momentum || momentum > self.traits.maximum_buy_momentum {
            return;
        }

        let amount_to_buy = calculate_amount_to_buy(money_to_spend, current_price);
        let money_spent_no_fee = amount_to_buy * current_price;

        let fee = money_spent_no_fee * config.transaction_fee_as_percentage;
        let money_spent = money_spent_no_fee + fee;

        let new_holding = CurrentHolding::new(current_price, current_price_data.time, amount_to_buy, money_spent, Asset::ETH, &self.traits, fee);

        self.current_holdings.push(new_holding);
        self.money -= money_spent;
    }

    fn handle_sell(&mut self, config: &Config, current_price_data: &PriceData) {
        for holding in &mut self.current_holdings {
            holding.update_for_new_period(current_price_data.close, &self.traits)
        }

        let mut sold_holdings: Vec::<SoldHolding> = Vec::<SoldHolding>::new();

        for holding in &mut self.current_holdings {
            // lambda that sells the holding
            let sell_holding = |reason: SellReason| -> SoldHolding {
                if reason == SellReason::TargetedSellPrice {
                    let sell_fee = calculate_sell_fee(holding.targeted_sell_price, config.transaction_fee_as_percentage, holding.amount);
                    let money_from_sell = calculate_money_from_sell(holding.targeted_sell_price, sell_fee, holding.amount);

                    return SoldHolding::new(&holding, holding.targeted_sell_price, money_from_sell, sell_fee, reason, current_price_data.time);
                }

                let sell_fee = calculate_sell_fee(current_price_data.close, config.transaction_fee_as_percentage, holding.amount);
                let money_from_sell = calculate_money_from_sell(current_price_data.close, sell_fee, holding.amount);

                return SoldHolding::new(&holding, current_price_data.close, money_from_sell, sell_fee, reason, current_price_data.time);
            };

            let sell_reason = get_sell_reason(&self.traits, &holding, &current_price_data);

            if sell_reason != SellReason::None {
                sold_holdings.push(sell_holding(sell_reason));
            }
        }

        // update the amount of money
        sold_holdings
            .iter()
            .for_each(|holding| self.money += holding.money_from_sell);
        self.sold_holdings.append(&mut sold_holdings);

        self.current_holdings = self.current_holdings.to_owned()
            .into_iter()
            .filter(|holding| {
                get_sell_reason(&self.traits, &holding, &current_price_data) == SellReason::None
            })
            .collect();
    }

    fn sell_all(&mut self, current_price_data: &PriceData, transaction_fee_as_percentage: f64) {
        for holding in &mut self.current_holdings {
            holding.update_for_new_period(current_price_data.close, &self.traits);

            let sell_fee = calculate_sell_fee(current_price_data.close, transaction_fee_as_percentage, holding.amount);
            let money_from_sell = calculate_money_from_sell(current_price_data.close, sell_fee, holding.amount);

            self.sold_holdings.push(SoldHolding::new(&holding, current_price_data.close, money_from_sell, sell_fee, SellReason::Forced, current_price_data.time));
            self.money += money_from_sell;
        }
        self.current_holdings.clear();

        // in sell all we update the fitness of the bot since it's meant to be the final run
        self.calculate_fitness();
    }

    // TODO: For now we will check every period
    // We will buy on open and sell on close
    // In the future we should set how often to buy and sell
    // In addition, we should set if to buy on open or close
    // Sell would occur on the flip? Or maybe be configurable by trait
    pub fn run_period(&mut self, price_history: &Arc<Vec<PriceData>>, period: u64, config: &Arc<Config>) {
        if self.start_time.is_none() {
            self.start_time = Some(price_history.get(0).unwrap().time);
        }

        if period < self.traits.number_of_averaging_periods {
            return;
        }

        let old_price_data = price_history.get((period - self.traits.number_of_averaging_periods) as usize).unwrap();
        let current_price_data = price_history.get((period) as usize).unwrap();

        // end of run
        if price_history.len() == (period + 1)  as usize {
            self.sell_all(&current_price_data, config.transaction_fee_as_percentage);
            self.end_time = Some(current_price_data.time);
            return;
        }

        self.handle_buy(&config, &current_price_data, &old_price_data);
        self.handle_sell(&config, &current_price_data);
    }

    // for now the calculation for fitness will be simple
    // we can work on a more complicated version once we have graphs in place
    fn calculate_fitness(&mut self) {
        if self.sold_holdings.len() == 0 {
            self.fitness = 0.0;
            return;
        }

        self.fitness = self.money;
    }

    // Hamming is the average percent difference between all traits
    // This is used for determining if the bots are to similar to breeed
    pub fn hamming(&self, bot_two: &Bot) -> f64 {
        let calculate_percent_difference = |value_one: f64, value_two: f64| -> f64 {
            let diff = (value_one - value_two) / ((value_one + value_two) / 2.0);

            return diff.abs() * 100.0;
        };

        let traits_one = self.traits;
        let traits_two = bot_two.traits;

        let number_of_averaging_periods_percent_diff = calculate_percent_difference(traits_one.number_of_averaging_periods as f64, traits_two.number_of_averaging_periods as f64);
        let minimum_buy_momentum_percent_diff = calculate_percent_difference(traits_one.minimum_buy_momentum as f64, traits_two.minimum_buy_momentum as f64);
        let maximum_buy_momentum_percent_diff = calculate_percent_difference(traits_one.maximum_buy_momentum as f64, traits_two.maximum_buy_momentum as f64);
        let trailing_stop_loss_percent_diff = calculate_percent_difference(traits_one.trailing_stop_loss as f64, traits_two.trailing_stop_loss as f64);
        let stop_loss_percent_diff = calculate_percent_difference(traits_one.stop_loss as f64, traits_two.stop_loss as f64);
        let minimum_holding_periods_percent_diff = calculate_percent_difference(traits_one.minimum_holding_periods as f64, traits_two.minimum_holding_periods as f64);
        let maximum_holding_periods_percent_diff = calculate_percent_difference(traits_one.maximum_holding_periods as f64, traits_two.maximum_holding_periods as f64);
        let percent_purchase_percent_diff = calculate_percent_difference(traits_one.percent_purchase as f64, traits_two.percent_purchase as f64);
        let target_sell_percentage_percent_diff = calculate_percent_difference(traits_one.target_sell_percentage as f64, traits_two.target_sell_percentage as f64);

        return (number_of_averaging_periods_percent_diff + minimum_buy_momentum_percent_diff + maximum_buy_momentum_percent_diff + trailing_stop_loss_percent_diff
        + stop_loss_percent_diff + minimum_holding_periods_percent_diff + maximum_holding_periods_percent_diff
        + percent_purchase_percent_diff + target_sell_percentage_percent_diff) / 9.0
    }

    pub fn breed<R: Rng>(&self, bot_two: &Bot, rng: &mut R, config: &Config, id: u64) -> Bot {
        let traits_one = self.traits;
        let traits_two = bot_two.traits;

        let number_of_averaging_periods = match rng.gen_bool(0.5) {
            true => traits_one.number_of_averaging_periods,
            false => traits_two.number_of_averaging_periods
        };

        let minimum_buy_momentum = match rng.gen_bool(0.5) {
            true => traits_one.minimum_buy_momentum,
            false => traits_two.minimum_buy_momentum
        };

        let maximum_buy_momentum = match rng.gen_bool(0.5) {
            true => traits_one.maximum_buy_momentum,
            false => traits_two.maximum_buy_momentum
        };

        let trailing_stop_loss = match rng.gen_bool(0.5) {
            true => traits_one.trailing_stop_loss,
            false => traits_two.trailing_stop_loss
        };

        let stop_loss = match rng.gen_bool(0.5) {
            true => traits_one.stop_loss,
            false => traits_two.stop_loss
        };

        let minimum_holding_periods = match rng.gen_bool(0.5) {
            true => traits_one.minimum_holding_periods,
            false => traits_two.minimum_holding_periods
        };

        let maximum_holding_periods = match rng.gen_bool(0.5) {
            true => traits_one.maximum_holding_periods,
            false => traits_two.maximum_holding_periods
        };

        let percent_purchase = match rng.gen_bool(0.5) {
            true => traits_one.percent_purchase,
            false => traits_two.percent_purchase
        };

        let target_sell_percentage = match rng.gen_bool(0.5) {
            true => traits_one.target_sell_percentage,
            false => traits_two.target_sell_percentage
        };

        let mut traits = Traits {
            number_of_averaging_periods,
            minimum_buy_momentum,
            maximum_buy_momentum,
            trailing_stop_loss,
            stop_loss,
            minimum_holding_periods,
            maximum_holding_periods,
            percent_purchase,
            target_sell_percentage
        };

        traits.mutate(rng, config);

        return Bot {
            id,
            traits,
            money: config.starting_money,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new(),
            fitness: 0.0,
            start_time: None,
            end_time: None
        };
    }
}
