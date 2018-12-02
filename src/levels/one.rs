use crate::prelude::*;

/// S -> top cheese beef cheese beef N bottom .
/// N -> cheese beef cheese beef N .
/// N -> .
pub struct DoubleCheeseburger;

impl Level for DoubleCheeseburger {
    fn name() -> &'static str {
        "DoubleCheeseburger"
    }

    fn description() -> &'static str {
        "Even number of patties"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, BottomBun),
            burger!(TopBun, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, BottomBun),
            burger!(TopBun, Beef, Beef, BottomBun),
        ]
    }
}