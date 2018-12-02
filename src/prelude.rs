use quicksilver;

pub use crate::ingredients::{Sprites};
pub use crate::animation::Animation;
pub use crate::burger::{BurgerAnimSeq, Burger};

pub use quicksilver::{
    Result, Error,
    combinators::{result, join_all},
    Future,
    load_file,
    geom::{Shape, Vector, Rectangle, Transform},
    graphics::{Background::Img, Color, Image, Font, FontStyle},
    lifecycle::{Asset, Settings, State, Event, Window, run},
    input::{ButtonState, MouseButton},
};

pub const SCALE: f32 = 3.75;

pub use std::rc::Rc;
pub use std::cell::RefCell;
