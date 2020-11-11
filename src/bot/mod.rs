pub mod traits;
pub mod holdings;
use traits::Traits;
use holdings::*;
use crate::config::Config;
use crate::asset::Asset;
use crate::price_data::PriceData;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionState {
    Buy,
    Sell,
    Netural
}

#[derive(Debug)]
pub struct Bot {
    pub state: ActionState,
    pub traits: Traits,
    pub money: f64,
    pub current_holdings: Vec<CurrentHolding>,
    pub sold_holdings: Vec<SoldHolding>
}

fn calculate_momentum(current_price: f64, previous_price: f64) -> f64 {
    ((current_price - previous_price) / previous_price) * 100.0
}

fn get_sell_reason(traits: &Traits, holding: &CurrentHolding, current_price_data: &PriceData) -> SellReason {
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

impl Bot {
    pub fn new(config: &Config) -> Bot {
        let mut bot = Bot {
            state: ActionState::Netural,
            traits: Traits::new(config),
            money: config.starting_money,
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new()
        };

        bot
    }

    fn handle_buy(&mut self, config: &Config, current_price_data: &PriceData) {
        let money_to_spend = self.money * self.traits.percent_purchase;
        if money_to_spend < config.minimum_purchase_size {
            return;
        }

        let fee = money_to_spend * config.fee;
        let purchase_amount = (money_to_spend - fee) * current_price_data.open;

        let new_holding = CurrentHolding::new(current_price_data.open, purchase_amount, money_to_spend, Asset::ETH, &self.traits, fee);

        self.current_holdings.push(new_holding);
        self.money -= money_to_spend;
    }

    fn handle_sell(&mut self, config: &Config, current_price_data: &PriceData) {
        for holding in &mut self.current_holdings {
            holding.update_for_new_period(current_price_data.close, &self.traits)
        }

        let mut sold_holdings: Vec::<SoldHolding> = Vec::<SoldHolding>::new();

        for holding in &mut self.current_holdings {
            let sell_holding = |reason: SellReason| -> SoldHolding {
                let sell_fee = current_price_data.close * config.fee;
                let money_from_sell = (current_price_data.close * holding.amount) - sell_fee;

                SoldHolding::new(&holding, current_price_data.close, money_from_sell, sell_fee, reason)
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

    // TODO: For now we will check every period
    // We will buy on open and sell on close
    // In the future we should set how often to buy and sell
    // In addition, we should set if to buy on open or close
    // Sell would occur on the flip? Of maybe be configurable by trait
    pub fn run_period(&mut self, price_history: &Vec<PriceData>, period: u64, config: &Config) {
        if period < self.traits.number_of_averaging_periods {
            return;
        }

        // verify purchase size
        if self.money < config.minimum_purchase_size {
            return;
        }

        // calculate the momentum
        let old_price_data = price_history.get((period - self.traits.number_of_averaging_periods) as usize).unwrap();
        let current_price_data = price_history.get((period - 1) as usize).unwrap();
        let momentum = calculate_momentum(current_price_data.open, old_price_data.open);

        self.handle_buy(&config, &current_price_data);

        // handle sell
        self.handle_sell(&config, &current_price_data);
    }
}