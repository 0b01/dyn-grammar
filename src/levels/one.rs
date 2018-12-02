use crate::prelude::*;

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