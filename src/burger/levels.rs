use crate::grammar::Token;
use super::BurgerItem;

trait Level {
    fn name() -> &'static str;
    fn description() -> &'static str;
}

pub enum Levels {
    /// even number of patties
    DoubleCheeseburger,
    /// a^n b a^n
    WcGangbang,
    /// a^n b^m c^l
    LandSeaAndAir,
}

/// S -> top cheese beef cheese beef N bottom .
/// N -> cheese beef cheese beef N .
/// N -> .
struct DoubleCheeseburger {
    examples: Vec<Token<BurgerItem>>,
}

impl DoubleCheeseburger {
    fn name() -> &'static str {
        "DoubleCheeseburger"
    }

    fn description() -> &'static str {
        "Even number of patties"
    }
}

/// S -> top N bottom .
/// N -> beef S beef .
/// N -> chicken .
/// S -> .
struct WcGangbang {
    examples: Vec<Token<BurgerItem>>,
}

impl Level for WcGangbang {

    fn name() -> &'static str {
        "El McGangbang"
    }

    fn description() -> &'static str {
        "The WcChicken is placed directly inside The Big Wack."
    }

}

///
struct LandSeaAndAir {
    examples: Vec<Token<BurgerItem>>,
}

impl Level for LandSeaAndAir {
    fn name() -> &'static str {
        "Land, Sea, and Air"
    }
    fn description() -> &'static str {
        "Beef, fish of filet, chicken. In that order"
    }
}