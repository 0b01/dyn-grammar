use crate::prelude::*;

pub struct LandSeaAndAir;

impl Level for LandSeaAndAir {
    fn name() -> &'static str {
        "Land, Sea, and Air"
    }

    fn description() -> &'static str {
        "Beef, fish of filet, chicken. In that order"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(TopBun, Beef, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Chicken, BottomBun),
            burger!(TopBun, Beef, Fish, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Fish, Chicken, BottomBun),
            burger!(TopBun, Beef, Fish, Fish, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Fish, Chicken, BottomBun),
            burger!(TopBun, Beef, Fish, Fish, Fish, Chicken, BottomBun),
            burger!(TopBun, Beef, Beef, Beef, Beef, Beef, Chicken, BottomBun),
        ]
    }
}