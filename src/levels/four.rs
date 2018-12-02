use crate::prelude::*;

/// S -> top beef beef N bottom .
/// N -> beef beef N .
/// N -> .
pub struct Four;

impl Level for Four {
    fn name() -> &'static str {
        "Addition"
    }

    fn description() -> &'static str {
        "(E + E)"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(Onion),
            burger!(Beef),
            burger!(TopBun, Onion, BBQ, Onion, BottomBun),
            burger!(TopBun, Onion, BBQ, Beef, BottomBun),
            burger!(TopBun, Beef, BBQ, Onion, BottomBun),
            burger!(TopBun, Beef, BBQ, Beef, BottomBun),
            burger!(TopBun, TopBun, Onion, BBQ, Onion, BottomBun, BBQ, Beef, BottomBun),
            burger!(TopBun, TopBun, Onion, BBQ, Onion, BottomBun, BBQ, Onion, BottomBun),
            burger!(TopBun, TopBun, Beef, BBQ, Onion, BottomBun, BBQ, Beef, BottomBun),
            burger!(TopBun, TopBun, Beef, BBQ, Onion, BottomBun, BBQ, Onion, BottomBun),
        ]
    }
}
/*

S -> E .
E -> Onion, .
E -> Beef, .
E -> LeftBun, E BBQ, E RightBun, .

*/
