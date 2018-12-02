use crate::prelude::*;
use crate::grammar::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum AbstractBurgerTree<T: Debug + Clone + PartialEq + Hash + Eq> {
    NonTerm((usize, Vec<Box<AbstractBurgerTree<T>>>)),
    Term(Token<T>),
    IncompleteParse,
    WrongToken,
    Cyclic,
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
    Success,
}

impl AbstractBurgerTree<BurgerItem> {
    pub fn to_burger(&self) -> Burger {
        let mut bg = Burger::new();
        bg.toks = self.to_burger_aux();
        bg
    }

    fn to_burger_aux(&self) -> Vec<Token<BurgerItem>> {
        use self::AbstractBurgerTree::*;
        let mut ret = vec![];
        match &self {
            Term(Token::Epsilon) => { }
            Term(t) => { ret.push(t.clone()); }
            NonTerm((_,t)) => {
                for i in t.iter() {
                    ret.extend(i.to_burger_aux());
                }
            }
            AdditionalTokens(i) => { ret.extend(i.to_burger_aux()); }
            _ => (),
        }
        ret
    }


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
            IncompleteParse | WrongToken | IncompleteParse | Cyclic => {
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

