use crate::prelude::*;

pub struct TutFour;

impl Level for TutFour {
    fn name() -> &'static str {
        "tutorial 4"
    }

    fn description() -> &'static str {
        "getting started 4"
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
