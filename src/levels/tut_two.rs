use crate::prelude::*;

pub struct TutTwo;

impl Level for TutTwo {
    fn name() -> &'static str {
        "tutorial 2"
    }

    fn description() -> &'static str {
        "getting started 2"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!{TopBun, Chicken, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, BottomBun},
        ]
    }
}