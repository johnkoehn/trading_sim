use std::collections::HashMap;
use crate::asset::Asset;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionState {
    Buy,
    Sell,
    Netural
}

#[derive(Debug)]
pub struct Bot {
    pub state: ActionState,
    pub assets: HashMap<Asset, f64>
}

impl Bot {
    pub fn new() -> Bot {
        let mut bot = Bot {
            state: ActionState::Netural,
            assets: HashMap::new()
        };

        for asset in Asset::iterator() {
            bot.assets.insert(*asset, 0.0);
        }

        bot
    }

    pub fn set_asset(&mut self, asset: Asset, amount: f64) {
        self.assets.insert(asset, amount);
    }
}