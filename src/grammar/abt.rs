// use crate::prelude::*;
use crate::grammar::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum AbstractBurgerTree<T: Debug + Clone + PartialEq + Hash + Eq> {
    NonTerm((u32, Vec<Box<AbstractBurgerTree<T>>>)),
    Term(Token<T>),
}

