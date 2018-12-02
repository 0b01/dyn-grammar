// use crate::prelude::*;
use crate::grammar::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum AbstractBurgerTree<T: Debug + Clone + PartialEq + Hash + Eq> {
    NonTerm((usize, Vec<Box<AbstractBurgerTree<T>>>)),
    Term(Token<T>),
    IncompleteParse,
    WrongToken,
    AdditionalTokens(Box<AbstractBurgerTree<T>>),
}

#[derive(Debug, Clone)]
pub enum AnimDelta {
    StepAnim,
    Incr,
    EnterPtr(usize),
    Noop,
    ExitPtr(usize),
    PauseIndefinitely,
}

impl<T: Debug + Clone + PartialEq + Hash + Eq> AbstractBurgerTree<T> {
    pub fn to_delta_seq(&self) -> Vec<AnimDelta> {
        use self::AbstractBurgerTree::*;
        let mut ret = vec![];
        match self {
            Term(Token::Epsilon) => {
                ret.push(AnimDelta::Noop);
            }
            Term(Token::Terminal(_t)) => {
                ret.push(AnimDelta::Incr);
                ret.push(AnimDelta::StepAnim);
            }
            Term(Token::NonTerminal(_)) => panic!("Impossible"),
            NonTerm(t) => {
                ret.push(AnimDelta::Incr);
                ret.push(AnimDelta::EnterPtr(t.0));
                for i in &t.1 {
                    ret.extend(i.to_delta_seq());
                }
                ret.push(AnimDelta::ExitPtr(t.0));
            }
            IncompleteParse | WrongToken |IncompleteParse => {
                ret.push(AnimDelta::PauseIndefinitely);
            }
            AdditionalTokens(i) => {
                ret.extend(i.to_delta_seq());
                ret.push(AnimDelta::PauseIndefinitely);
            }
        }

        ret
    }
}

