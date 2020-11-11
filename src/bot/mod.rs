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
    pub assets: HashMap<Asset, f64>,
    pub traits: Traits,
    pub current_holdings: Vec<CurrentHolding>,
    pub sold_holdings: Vec<SoldHolding>
}

fn calculate_momentum(current_price: f64, previous_price: f64) -> f64 {
    ((current_price - previous_price) / previous_price) * 100.0
}

impl Bot {
    pub fn new(config: &Config) -> Bot {
        let mut bot = Bot {
            state: ActionState::Netural,
            assets: HashMap::new(),
            traits: Traits::new(config),
            current_holdings: Vec::<CurrentHolding>::new(),
            sold_holdings: Vec::<SoldHolding>::new()
        };

        for asset in Asset::iterator() {
            bot.assets.insert(*asset, 0.0);
        }

        bot.assets.insert(Asset::USD, config.starting_money);

        bot
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
        if self.assets.get(&Asset::USD).unwrap() < &config.minimum_purchase_size {
            return;
        }

        // calculate the momentum
        let old_price_data = price_history.get((period - self.traits.number_of_averaging_periods) as usize).unwrap();
        let current_price_data = price_history.get((period - 1) as usize).unwrap();
        let momentum = calculate_momentum(current_price_data.open, old_price_data.open);

        if self.traits.minimum_buy_momentum <= momentum {

        }

        // Update all current holdings
        // If any holdings need to be sold, combine and sell them
        // convert sold holdings to sold holdings, lol
        // we'll need to filter out old holdings
        // add a field to do so


        // transact
    }
}