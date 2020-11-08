pub mod traits;
use traits::Trait;
use std::collections::HashMap;
use crate::asset::Asset;
use serde_json::{Value, Map, Number};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionState {
    Buy,
    Sell,
    Netural
}

pub struct Bot {
    pub state: ActionState,
    pub assets: HashMap<Asset, f64>,
    pub traits: HashMap<Trait, Value>
}

impl Bot {
    pub fn new() -> Bot {
        let mut bot = Bot {
            state: ActionState::Netural,
            assets: HashMap::new(),
            traits: HashMap::new()
        };

        for asset in Asset::iterator() {
            bot.assets.insert(*asset, 0.0);
        }

        bot.traits.insert(Trait::BuyDirection,  Value::from(5));
        let test = Value::from(5.0);
        let test2 = Value::from(5);
        // test2.

        bot
    }

    pub fn set_asset(&mut self, asset: Asset, amount: f64) {
        self.assets.insert(asset, amount);
    }
}