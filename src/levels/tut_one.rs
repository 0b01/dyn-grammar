use crate::prelude::*;

pub struct TutOne;

impl Level for TutOne {
    fn name() -> &'static str {
        "tutorial 2"
    }

    fn description() -> &'static str {
        "getting started 2"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, Cheese, Beef, BottomBun),
            burger!(Lettuce, Beef, Lettuce),
            burger!(TopBun, Cheese, Beef, BottomBun),
            burger!(Lettuce, Beef, Lettuce),
            burger!(TopBun, Cheese, Beef, BottomBun),
            burger!(Lettuce, Beef, Lettuce),
            burger!(TopBun, Cheese, Beef, BottomBun),
            burger!(Lettuce, Beef, Lettuce),
            burger!(TopBun, Cheese, Beef, BottomBun),
            burger!(Lettuce, Beef, Lettuce),
        ]
    }
}