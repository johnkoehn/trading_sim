use crate::config;
use config::Config;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Both
}

// TODO: Remove unused traits from the bot in the simulation - i.e. bots become more scoped over time
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Traits {
    pub number_of_averaging_periods: u64,
    pub minimum_buy_momentum: f64,
    pub maximum_buy_momentum: f64,
    pub trailing_stop_loss: f64,
    pub stop_loss: f64,
    pub minimum_holding_periods: u64,
    pub maximum_holding_periods: u64,
    pub percent_purchase: f64,
    pub target_sell_percentage: f64
}

impl Traits {
    pub fn new(config: &Config) -> Traits {
        let mut rng = rand::thread_rng();

        let number_of_averaging_periods = rng.gen_range(config.traits.number_of_averaging_periods.min, config.traits.number_of_averaging_periods.max);
        let minimum_buy_momentum = rng.gen_range(config.traits.minimum_buy_momentum.min, config.traits.minimum_buy_momentum.max);
        let maximum_buy_momentum = rng.gen_range(config.traits.maximum_buy_momentum.min, config.traits.maximum_buy_momentum.max);
        let trailing_stop_loss = rng.gen_range(config.traits.trailing_stop_loss.min, config.traits.trailing_stop_loss.max);
        let stop_loss = rng.gen_range(config.traits.stop_loss.min, config.traits.stop_loss.max);
        let minimum_holding_periods = rng.gen_range(config.traits.minimum_holding_periods.min, config.traits.minimum_holding_periods.max);
        let maximum_holding_periods = rng.gen_range(config.traits.maximum_holding_periods.min, config.traits.maximum_holding_periods.max);
        let percent_purchase = rng.gen_range(config.traits.percent_purchase.min, config.traits.percent_purchase.max);
        let target_sell_percentage = rng.gen_range(config.traits.target_sell_percentage.min, config.traits.target_sell_percentage.max);

        Traits {
            number_of_averaging_periods,
            minimum_buy_momentum,
            maximum_buy_momentum,
            trailing_stop_loss,
            stop_loss,
            minimum_holding_periods,
            maximum_holding_periods,
            percent_purchase,
            target_sell_percentage
        }
    }

    pub fn mutate<R: Rng>(&mut self, rng: &mut R, config: &Config) {
        // for each trait, see if we need to regenerate based on mutation chance
        self.number_of_averaging_periods = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.number_of_averaging_periods.min, config.traits.number_of_averaging_periods.max),
            false => self.number_of_averaging_periods
        };
        self.minimum_buy_momentum = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.minimum_buy_momentum.min, config.traits.minimum_buy_momentum.max),
            false => self.minimum_buy_momentum
        };
        self.maximum_buy_momentum = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.maximum_buy_momentum.min, config.traits.maximum_buy_momentum.max),
            false => self.maximum_buy_momentum
        };
        self.trailing_stop_loss = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.trailing_stop_loss.min, config.traits.trailing_stop_loss.max),
            false => self.trailing_stop_loss
        };
        self.stop_loss = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.stop_loss.min, config.traits.stop_loss.max),
            false => self.stop_loss
        };
        self.minimum_holding_periods = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.minimum_holding_periods.min, config.traits.minimum_holding_periods.max),
            false => self.minimum_holding_periods
        };
        self.maximum_holding_periods = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.maximum_holding_periods.min, config.traits.maximum_holding_periods.max),
            false => self.maximum_holding_periods
        };
        self.percent_purchase = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.percent_purchase.min, config.traits.percent_purchase.max),
            false => self.percent_purchase
        };
        self.target_sell_percentage = match rng.gen_bool(config.mutation_chance) {
            true => rng.gen_range(config.traits.target_sell_percentage.min, config.traits.target_sell_percentage.max),
            false => self.target_sell_percentage
        };
    }
}

// NOTE: Not bots get all traits. A bot can lose any set of traits due to random mutation
// a mutation on a essential trait casues the bot to sucide -- not pass on traits