pub mod traits;
use traits::Traits;
use crate::config::Config;
use crate::asset::Asset;
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
    pub traits: Traits
}

impl Bot {
    pub fn new(config: &Config) -> Bot {
        let mut bot = Bot {
            state: ActionState::Netural,
            assets: HashMap::new(),
            traits: Traits::new(config)
        };

        for asset in Asset::iterator() {
            bot.assets.insert(*asset, 0.0);
        }

        bot.assets.insert(Asset::USD, config.starting_money);

        bot
    }

    pub fn set_asset(&mut self, asset: Asset, amount: f64) {
        self.assets.insert(asset, amount);
    }
}