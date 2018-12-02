use crate::prelude::*;

use crate::burger::*;
use crate::game::{Orders, GameBurgerRule};
use crate::grammar::*;
use crate::levels;

pub struct Game {
    pub rules: Vec<GameBurgerRule>,
    pub burg_anim: Option<BurgerAnimSeq>,
    pub seq: Option<Vec<AnimDelta>>,
    pub orders: Orders,
    pub level: usize,
    pub continuous: bool,
    pub rule_stack: Vec<usize>,
    pub pause: bool,

    pub is_debugging: bool,

    pub step_pressed: bool,
    pub stop_pressed: bool,
    pub play_pressed: bool,
}

impl Game {

    pub fn new(rules: Vec<GameBurgerRule>) -> Self {
        Self {
            rules,
            burg_anim: Option::None,
            orders: Orders::new(),
            seq: Option::None,
            level: 0,
            rule_stack: Vec::new(),
            pause: false,
            continuous: false,
            step_pressed: false,
            stop_pressed: false,
            play_pressed: false,
            is_debugging: false,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&GameBurgerRule> {
        self.rules.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut GameBurgerRule> {
        self.rules.get_mut(idx)
    }

    pub fn set_level(&mut self, i: usize) {
        self.level = i;
        match self.level {
            0 => { self.orders.orders = Some(levels::TutOne::orders()); }
            1 => { self.orders.orders = Some(levels::TutTwo::orders()); }
            2 => { self.orders.orders = Some(levels::TutThree::orders()); }
            3 => { self.orders.orders = Some(levels::TutFour::orders()); }
            4 => { self.orders.orders = Some(levels::Two::orders()); }
            5 => { self.orders.orders = Some(levels::Four::orders()); }
            6 => { self.orders.orders = Some(levels::One::orders()); }
            7 => { self.orders.orders = Some(levels::Three::orders()); }
            _ => unimplemented!(),
        }
    }

    pub fn drop_item(&mut self, v: &Vector, itm: Option<BurgerItem>) {
        if !self.rule_stack.is_empty() {return;}
        for grammar in &mut self.rules {
            let itm = itm.unwrap_or(BurgerItem::None); // remove item if empty
            grammar.set_item_with_pos(v, itm);
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {

        let game_ui = if self.is_debugging {
            ing.get_img("game_ui_light").unwrap()
        } else {
            ing.get_img("game_ui").unwrap()
        };
        window.draw(&game_ui.area(), Img(&game_ui));

        if self.burg_anim.is_some() {
            let burger = self.burg_anim.as_mut().unwrap();
            burger.draw(window, ing)?;
        }

        if self.continuous && !self.is_anim_playing() {
            self.step_burger(ing)?;
        }

        for grammar in &mut self.rules {
            grammar.draw(window, ing)?;
        }
        self.draw_btn(window, ing)?;
        self.orders.draw(window, ing)?;
        Ok(())
    }

    fn draw_btn(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        let image = if self.pause {
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

        let image = if self.pause {
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

    pub fn build(&mut self) -> Result<()>{

        let i = self.orders.selected;

        let canonical_bg = &self.orders.orders.as_ref().unwrap()[i];
        // let anim = BurgerAnimSeq::new(bg.clone());
        // self.burg_anim = Some(anim.clone());

        let mut g = self.as_grammar();

        let abt = match g.build() {
            Err(e) => e,
            Ok(()) => {
                g.parse(canonical_bg.toks.clone())
                    .unwrap_or_else(|t|t)
            }
        };
        let my_bg = abt.to_burger();
        // println!("{:#?}", abt);
        // println!("{:#?}", my_bg);


        let anim = BurgerAnimSeq::new(my_bg.clone());
        self.burg_anim = Some(anim.clone());

        // println!("{:#?}", abt);
        let mut seq = abt.to_delta_seq();
        if &my_bg != canonical_bg {
            seq.push(AnimDelta::PauseIndefinitely);
        } else {
            seq.push(AnimDelta::Success);
        }
        self.seq = Some(seq);
        Ok(())
    }

    pub fn play_burger(&mut self, ingr: &mut Sprites) -> Result<()> {
        // println!("fn play burger");
        self.stop_burger(ingr)?;
        self.build()?;
        ingr.set_duration(0.14)?;
        self.continuous = true;

        if !self.is_anim_playing() {
            self.step_burger(ingr)?;
        }
        Ok(())
    }

    pub fn stop_burger(&mut self, _ingr: &mut Sprites) -> Result<()> {
        // println!("fn stop burger");
        for i in &mut self.rules { i.pointer.clear(); }
        self.rule_stack.clear();
        self.pause = false;
        self.is_debugging = false;
        self.continuous = false;
        self.play_pressed = false;
        self.build()?;
        Ok(())
    }

    pub fn is_anim_playing(&mut self) -> bool {
        self.burg_anim.as_ref().unwrap().drawing.is_some()
    }

    pub fn step_burger(&mut self, ingr: &mut Sprites) -> Result<()> {
        // println!("fn step burger");
        if self.pause { return Ok(()) }
        if !self.is_debugging { self.build()? }
        if self.is_anim_playing() { return Ok(()); }
        use self::AnimDelta::*;

        let seq = self.seq.as_mut().unwrap();
        let delta = seq.first().cloned();
        if delta.is_none() {
            self.continuous = false;
            return Ok(())
        }
        let delta = delta.unwrap();
        drop(seq);
        // println!("{:#?}", delta);
        match delta {
            Incr => {
                let rule_id = self.rule_stack.last();
                match rule_id {
                    Some(rule_id) => {
                        let rule = self.get_mut(*rule_id).unwrap();
                        rule.step();
                    }
                    Option::None => {
                        self.is_debugging = true;
                    },
                }
            }
            StepAnim => {
                let burger_seq = self.burg_anim.as_mut().unwrap();
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
            PauseIndefinitely => {
                self.pause = true;
            }
            Success => {
                println!("SUCCESS!");
                self.orders.set_success(self.orders.selected);

                if self.orders.selected + 1 == 10 {
                    self.set_level(self.level + 1);
                    self.orders.selected = 0;
                    self.orders.clear_result();
                } else{
                    self.orders.selected += 1;
                    if self.continuous {
                        self.play_burger(ingr)?;
                    } else {
                        self.stop_burger(ingr)?;
                    }
                }
            }
        };

        drop(delta);
        let seq = self.seq.as_mut().unwrap();
        seq.remove(0);

        Ok(())
    }

    pub fn mouse_move(&mut self, v: &Vector) {
        if self.is_debugging { return; }
        match self.orders.mouse_move(v) {
            Option::None => (),
            Some(_) => (),
        };
    }
}
