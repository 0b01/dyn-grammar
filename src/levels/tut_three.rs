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
            burger!{TopBun, Chicken, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun},
            burger!{TopBun, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Chicken, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, Fish, BottomBun }
        ]
    }
}
