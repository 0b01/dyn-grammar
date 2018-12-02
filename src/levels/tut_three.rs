use crate::prelude::*;

pub struct TutThree;

impl Level for TutThree {
    fn name() -> &'static str {
        "tutorial 3"
    }

    fn description() -> &'static str {
        "getting started 3"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, Beef, BottomBun),
        ]
    }
}


/*

S -> TopBun, A BottomBun, .
A -> Ketchup, Mayo, .
A -> Beef, A .

*/