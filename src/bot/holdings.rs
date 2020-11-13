use crate::asset::Asset;
use crate::bot::Traits;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SellReason {
    StopLoss,
    TrailingStopLoss,
    MaxPeriodsHeld,
    TargetedSellPrice, // TODO: Future trait to implement
    Forced,
    None
}

#[derive(Debug, Copy, Clone)]
pub struct CurrentHolding {
    pub asset: Asset,
    pub amount: f64,
    pub money_spent: f64,
    pub purchase_price: f64,
    pub stop_loss: f64, // the price to stop out at
    pub trailing_stop_loss: f64, // current trailing price to stop out at
    pub periods_held: u64,
    pub buy_fee: f64
}

fn calculate_stop_loss(price: f64, stop_loss_percentage: f64) -> f64 {
    price - (price * (stop_loss_percentage / 100.0))
}

fn calculate_percent_gained(money_spent: f64, money_from_sell: f64) -> f64 {
    ((money_from_sell - money_spent) / money_spent) * 100.0
}

impl CurrentHolding {
    pub fn new(purchase_price: f64, amount: f64, money_spent: f64, asset: Asset, traits: &Traits, buy_fee: f64) -> CurrentHolding {
        CurrentHolding {
            asset,
            amount,
            money_spent,
            purchase_price,
            stop_loss: calculate_stop_loss(purchase_price, traits.stop_loss),
            trailing_stop_loss: calculate_stop_loss(purchase_price, traits.trailing_stop_loss),
            periods_held: 0,
            buy_fee
        }
    }

    pub fn update_for_new_period(&mut self, current_price: f64, traits: &Traits) {
        let trailing_stop_loss = calculate_stop_loss(current_price, traits.trailing_stop_loss);

        if trailing_stop_loss > self.trailing_stop_loss {
            self.trailing_stop_loss = trailing_stop_loss;
        }

        self.periods_held += 1;
    }
}

#[derive(Debug)]
pub struct SoldHolding {
    pub asset: Asset,
    pub amount: f64,
    pub purchase_price: f64,
    pub money_spent: f64,
    pub periods_held: u64,
    pub sell_reason: SellReason,
    pub sell_price: f64,
    pub percent_gained: f64,
    pub amount_gained: f64,
    pub money_from_sell: f64,
    pub win: bool,
    pub buy_fee: f64,
    pub sell_fee: f64
}

impl SoldHolding {
    pub fn new(holding_sold: &CurrentHolding, sell_price: f64, money_from_sell: f64, sell_fee: f64, sell_reason: SellReason) -> SoldHolding {
        SoldHolding {
            asset: holding_sold.asset,
            amount: holding_sold.amount,
            purchase_price: holding_sold.purchase_price,
            money_spent: holding_sold.money_spent,
            periods_held: holding_sold.periods_held,
            sell_reason,
            sell_price,
            money_from_sell,
            percent_gained: calculate_percent_gained(holding_sold.money_spent, money_from_sell),
            amount_gained: money_from_sell - holding_sold.money_spent,
            win: (money_from_sell - holding_sold.money_spent) > 0.0,
            buy_fee: holding_sold.buy_fee,
            sell_fee
        }
    }
}