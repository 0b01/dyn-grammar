use crate::prelude::*;

use crate::burger::*;
use crate::grammar::*;

use self::BurgerItem::*;

const TITLE_HEIGHT: f32 = 50.;
const LINE_HEIGHT: f32 = 45.;
const HEIGHT: f32 = 260.;
const WIDTH: f32 = 70.;

pub struct GameGrammar {
    pub name: BurgerItem,
    pub items: [BurgerItem; 5],
    pub top_left: Vector,
    pub id: u32,
}

pub struct Game {
    rules: Vec<GameGrammar>,
}

impl GameGrammar {
    pub fn new(top_left: Vector, id: u32) -> Self {
        Self {
            name: None,
            items: [None; 5],
            top_left,
            id,
        }
    }

    pub fn set_name(&mut self, i: BurgerItem) {
        self.name = i;
    }

    pub fn set_item(&mut self, idx: usize, i: BurgerItem) {
        assert!(idx < 5);
        self.items[idx] = i;
    }

    pub fn set_item_with_pos(&mut self, p: &Vector, i: BurgerItem) {
        // if i.is_nonterm() {
        //     if p.x > self.top_left.x && p.x < self.top_left.x + WIDTH
        //     && p.y > self.top_left.y && p.y < self.top_left.y + HEIGHT {
        //         self.name = i;
        //     }
        // }

        if p.x > self.top_left.x && p.x < self.top_left.x + WIDTH
        && p.y > self.top_left.y && p.y < self.top_left.y + HEIGHT {
            let ratio = (p.y - self.top_left.y - TITLE_HEIGHT) / (HEIGHT - TITLE_HEIGHT);
            let nth = (ratio * 5.).floor();
            if nth == -1. && (i.is_nonterm() || i == None) {
                self.name = i;
            }
            if nth >= 0. {
                self.items[nth as usize] = i;
            }
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &Ingredients) -> Result<()> {
        let title = self.name.to_str();
        if title != "" {
            let image = ing.get_img(title).unwrap();
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(self.top_left.x + 16., self.top_left.y + 4.),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
        }

        for (i, item) in self.items.iter().enumerate() {
            let name = item.to_str();
            if name == "" { continue; }
            let img = ing.get_img(name).unwrap();
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(
                        self.top_left.x + 16.,
                        self.top_left.y + TITLE_HEIGHT + 4. + i as f32 * LINE_HEIGHT),
                    Vector::new(32., 32.)
                ),
                Img(&img),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
        }
        Ok(())
    }

    pub fn as_rule(&self) -> Option<Rule<BurgerItem>> {
        if self.name == None {
            return ::std::option::Option::None;
        }
        let mut production = vec![];
        for i in &self.items {
            if *i == None {
                production.push(Token::Epsilon);
            } else if i.is_nonterm() {
                production.push(Token::NonTerminal(i.to_str().to_owned()));
            } else {
                production.push(Token::Terminal(*i))
            }
        }

        Some(Rule {
            name: self.name.to_str().to_owned(),
            production,
            id: self.id,
        })

    }

}

impl Game {

    pub fn new(rules: Vec<GameGrammar>) -> Self {
        Self {
            rules,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&GameGrammar> {
        self.rules.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut GameGrammar> {
        self.rules.get_mut(idx)
    }

    pub fn drop_item(&mut self, v: &Vector, itm: Option<BurgerItem>) {
        for grammar in &mut self.rules {
            let itm = itm.unwrap_or(BurgerItem::None); // remove item if empty
            grammar.set_item_with_pos(v, itm);
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &Ingredients) -> Result<()> {
        for grammar in &mut self.rules {
            grammar.draw(window, ing)?;
        }
        Ok(())
    }

    pub fn as_grammar(&self) -> Grammar<BurgerItem> {
        let rules = self.rules.iter()
            .map(|i| i.as_rule())
            .filter(|i|i.is_some())
            .map(|i|i.unwrap())
            .collect();

        Grammar::new(
            "S".to_owned(),
            rules,
        )

        // g.build().unwrap();

    }

}
