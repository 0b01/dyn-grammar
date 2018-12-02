use crate::prelude::*;

/// S -> top beef beef N bottom .
/// N -> beef beef N .
/// N -> .
pub struct One;

impl Level for One {
    fn name() -> &'static str {
        "DoubleCheeseburger"
    }

    fn description() -> &'static str {
        "Even number of patties"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
        ]
    }
}