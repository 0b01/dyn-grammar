use crate::prelude::*;

use crate::burger::*;
use crate::game::{Orders, GameBurgerRule};
use crate::grammar::*;


pub struct Game {
    rules: Vec<GameBurgerRule>,
    burger: Option<BurgerAnimSeq>,
    orig_burger: Option<BurgerAnimSeq>,
    seq: Option<Vec<AnimDelta>>,
    orders: Orders,

    rule_stack: Vec<usize>,

    pub step_pressed: bool,
    pub stop_pressed: bool,
    pub play_pressed: bool,
}

impl Game {

    pub fn new(rules: Vec<GameBurgerRule>) -> Self {
        Self {
            rules,
            burger: Option::None,
            orig_burger: Option::None,
            orders: Orders::new(),
            seq: Option::None,
            rule_stack: Vec::new(),
            step_pressed: false,
            stop_pressed: false,
            play_pressed: false,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&GameBurgerRule> {
        self.rules.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut GameBurgerRule> {
        self.rules.get_mut(idx)
    }

    pub fn drop_item(&mut self, v: &Vector, itm: Option<BurgerItem>) {
        if !self.rule_stack.is_empty() {return;}
        for grammar in &mut self.rules {
            let itm = itm.unwrap_or(BurgerItem::None); // remove item if empty
            grammar.set_item_with_pos(v, itm);
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        if self.burger.is_some() {
            let burger = self.burger.as_mut().unwrap();
            burger.draw(window, ing)?;
        }
        for grammar in &mut self.rules {
            grammar.draw(window, ing)?;
        }
        self.draw_btn(window, ing)?;
        self.orders.draw(window, ing)?;
        Ok(())
    }

    fn draw_btn(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {

        let image = if self.seq.as_ref().unwrap().is_empty() {
            ing.get_img("stepdisabled").unwrap()
        } else if self.step_pressed {
            ing.get_img("stepdown").unwrap()
        } else {
            ing.get_img("stepup").unwrap()
        };
        window.draw_ex(&
            Rectangle::new(
                Vector::new(600., 380.),
                Vector::new(32., 32.)
            ),
            Img(&image),
            Transform::scale(Vector::new(1.5, 1.5)),
            100,
        );

        let image = if self.seq.as_ref().unwrap().is_empty() {
            ing.get_img("playdisabled").unwrap()
        } else if self.play_pressed {
            ing.get_img("playdown").unwrap()
        } else {
            ing.get_img("playup").unwrap()
        };
        window.draw_ex(&
            Rectangle::new(
                Vector::new(600.+ 48., 380.),
                Vector::new(32., 32.)
            ),
            Img(&image),
            Transform::scale(Vector::new(1.5, 1.5)),
            100,
        );

        let image = if self.stop_pressed {
            ing.get_img("stopdown").unwrap()
        } else {
            ing.get_img("stopup").unwrap()
        };
        window.draw_ex(&
            Rectangle::new(
                Vector::new(600.+ 48. + 48., 380.),
                Vector::new(32., 32.)
            ),
            Img(&image),
            Transform::scale(Vector::new(1.5, 1.5)),
            100,
        );


        Ok(())
    }


    pub fn as_grammar(&self) -> Grammar<BurgerItem> {
        let rules = self.rules.iter().enumerate()
            .map(|(idx, i)| i.as_rule(idx as usize))
            .filter(|i|i.is_some())
            .map(|i|i.unwrap())
            .collect();

        Grammar::new(
            BurgerItem::NonTermS.to_str().to_owned(),
            rules,
        )
    }

    pub fn set_burger(&mut self, b: &BurgerAnimSeq) -> Result<()> {
        self.orig_burger = Some(b.clone());
        self.burger = Some(b.clone());
        let mut g = self.as_grammar();
        g.build().unwrap();
        let abt = g.parse(b.burger.toks.clone()).unwrap();
        self.seq = Some(abt.to_delta_seq());
        Ok(())
    }

    pub fn play_burger(&mut self, ingr: &mut Sprites) -> Result<()> {
        if self.seq.as_ref().unwrap().is_empty() { return Ok(()) }
        let bg = self.burger.as_mut().unwrap();
        self.seq = Some(Vec::new());
        bg.cont(ingr)
    }

    pub fn stop_burger(&mut self, ingr: &mut Sprites) -> Result<()> {
        for i in &mut self.rules {
            i.pointer.clear();
        }
        self.rule_stack.clear();
        self.burger.as_mut().unwrap().stop(ingr);
        self.play_pressed = false;
        let bg = self.orig_burger.as_ref().unwrap().clone();
        self.set_burger(&bg)?;
        Ok(())
    }

    pub fn step_burger(&mut self, ingr: &mut Sprites) -> Result<()> {
        if self.burger.as_ref().unwrap().drawing.is_some() { return Ok(()); }
        use self::AnimDelta::*;

        ingr.set_duration(0.4)?;

        if self.burger.is_none() { return Ok(()); }
        let seq = self.seq.as_mut().unwrap();
        let delta = seq.first().cloned();
        if delta.is_none() {return Ok(())}
        let delta = delta.unwrap();
        drop(seq);
        println!("{:#?}", delta);
        match delta {
            Incr => {
                let rule_id = self.rule_stack.last();
                match rule_id {
                    Some(rule_id) => {
                        let rule = self.get_mut(*rule_id).unwrap();
                        rule.step();
                    }
                    Option::None => (),
                }
            }
            StepAnim => {
                let burger_seq = self.burger.as_mut().unwrap();
                burger_seq.step(ingr)?;
            }
            EnterPtr(id) => {
                self.rule_stack.push(id);
                let rule = self.get_mut(id).unwrap();
                rule.pointer.push(5);
            },
            Noop => {
                //
            },
            ExitPtr(id) => {
                self.rule_stack.pop();
                let rule = self.get_mut(id).unwrap();
                rule.pointer.pop();
            },
        };
        drop(delta);
        let seq = self.seq.as_mut().unwrap();
        seq.remove(0);

        Ok(())
    }

    pub fn mouse_move(&mut self, v: &Vector) {
        self.orders.mouse_move(v);
    }
}
