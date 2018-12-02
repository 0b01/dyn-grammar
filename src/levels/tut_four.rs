use crate::prelude::*;

pub struct TutFour;

impl Level for TutFour {
    fn name() -> &'static str {
        "British Delight"
    }

    fn description() -> &'static str {
        "Mayo goodness and ... lots of bread"
    }

    fn orders() -> [Burger; 10] {
        [
            burger![Mayo],
            burger![TopBun, Mayo, Mayo, BottomBun],
            burger![TopBun, TopBun, Mayo, Mayo, BottomBun, Mayo, BottomBun],
            burger![TopBun, Mayo, TopBun, Mayo, Mayo, BottomBun, BottomBun],
            burger![TopBun, TopBun, TopBun, Mayo, Mayo, BottomBun, Mayo, BottomBun, Mayo, BottomBun],
            burger![TopBun, TopBun, Mayo, TopBun, Mayo, Mayo, BottomBun, BottomBun, Mayo, BottomBun],
            burger![TopBun, TopBun, Mayo, Mayo, BottomBun, TopBun, Mayo, Mayo, BottomBun, BottomBun],
            burger![TopBun, Mayo, TopBun, TopBun, Mayo, Mayo, BottomBun, Mayo, BottomBun, BottomBun],
            burger![TopBun, Mayo, TopBun, Mayo, TopBun, Mayo, Mayo, BottomBun, BottomBun, BottomBun],
            burger![TopBun, Mayo, TopBun, TopBun, Mayo, Mayo, BottomBun, TopBun, Mayo, Mayo, BottomBun, BottomBun, BottomBun],

        ]
    }
}

/*

S -> TopBun, A BottomBun, .
S -> Mayo, .
A -> S S .

*/