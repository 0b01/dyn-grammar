use crate::prelude::*;

///
struct LandSeaAndAir {
    examples: Vec<Token<BurgerItem>>,
}

impl Level for LandSeaAndAir {
    fn name() -> &'static str {
        "Land, Sea, and Air"
    }

    fn description() -> &'static str {
        "Beef, fish of filet, chicken. In that order"
    }

    fn orders() -> [Burger; 10] {
        [
            burger!(Beef, Chicken),
            burger!(Beef, Beef, Chicken),
            burger!(Beef, Fish, Chicken),
            burger!(Beef, Beef, Beef, Chicken),
            burger!(Beef, Beef, Fish, Chicken),
            burger!(Beef, Fish, Fish, Chicken),
            burger!(Beef, Beef, Beef, Beef, Chicken),
            burger!(Beef, Beef, Beef, Fish, Chicken),
            burger!(Beef, Fish, Fish, Fish, Chicken),
            burger!(Beef, Beef, Beef, Beef, Beef, Chicken),
        ]
    }
}