pub mod tut_one;
pub mod tut_two;
pub mod tut_three;
pub mod one;
pub mod two;
pub mod three;

use crate::prelude::*;

pub trait Level {
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn orders() -> [Burger; 10];
}

pub enum Levels {
    /// even number of patties
    DoubleCheeseburger,
    /// a^n b a^n
    WcGangbang,
    /// a^n b^m c^l
    LandSeaAndAir,
}
