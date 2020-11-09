#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Both
}

// TODO: Remove unused traits from the bot in the simulation - i.e. bots become more scoped over time
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Trait {
    NumberOfAveragingPeriods, // the periods of data points to look at to derive averagings
    BuyDirection,
    SellDirection,
    MinimumBuyMomentum, // Always a positive value, it acts negative when buydirection is true (decimal)
    MaximumBuyMomentum,
    TrailingStopLoss,
    StopLoss,
    MaxHoldingPeriods,
    MinimumHoldingPeriods,
    MaximumHoldingPeriods
}

// NOTE: Not bots get all traits. A bot can lose any set of traits due to random mutation
// a mutation on a essential trait casues the bot to sucide -- not pass on traits