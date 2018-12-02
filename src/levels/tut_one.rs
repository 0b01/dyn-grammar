use crate::prelude::*;

pub struct TutOne;

impl Level for TutOne {
    fn name() -> &'static str {
        "tutorial 1"
    }

    fn description() -> &'static str {
        "getting started"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
            burger!(TopBun, Beef, Lettuce, Ketchup, BottomBun),
        ]
    }
}