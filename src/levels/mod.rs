pub mod tut_one;
pub mod tut_two;
pub mod tut_three;
pub mod tut_four;
pub mod one;
pub mod two;
pub mod three;

pub use self::tut_one::*;
pub use self::tut_two::*;
pub use self::tut_three::*;
pub use self::tut_four::*;
pub use self::one::*;
pub use self::two::*;
pub use self::three::*;


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


/*



*/