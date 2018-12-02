use crate::prelude::*;
use self::BurgerItem::*;
use crate::grammar::*;

const TITLE_HEIGHT: f32 = 50.;
const LINE_HEIGHT: f32 = 45.;
const HEIGHT: f32 = 260.;
const WIDTH: f32 = 70.;


pub struct GameBurgerRule {
    pub name: BurgerItem,
    pub items: [BurgerItem; 5],
    pub top_left: Vector,
    pub pointer: Vec<usize>,
}

impl GameBurgerRule {
    pub fn new(top_left: Vector) -> Self {
        Self {
            name: None,
            items: [None; 5],
            top_left,
            pointer: vec![],
        }
    }

    pub fn set_name(&mut self, i: BurgerItem) {
        self.name = i;
    }

    pub fn set_item(&mut self, idx: usize, i: BurgerItem) {
        assert!(idx < 5);
        self.items[idx] = i;
    }

    pub fn set_item_with_pos(&mut self, p: &Vector, i: BurgerItem) -> bool {
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
            if nth == -1. && (i.is_nonterm() || i == None)
            && !(self.top_left.x == 45. && self.top_left.y == 20.) { // cannot be top left
                self.name = i;
                return true
            }
            if nth >= 0. {
                self.items[nth as usize] = i;
                return true
            }
        }
        return false;
    }

    pub fn draw(&mut self, window: &mut Window, ing: &Sprites) -> Result<()> {
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

        if !self.pointer.is_empty() {
            let img = ing.get_img("pointer").unwrap();
            let i = *self.pointer.last().unwrap();
            let (px, py) = if i == 5 {
                (
                    self.top_left.x - 32.,
                    self.top_left.y,
                )
            } else {
                (
                    self.top_left.x - 32.,
                    self.top_left.y + TITLE_HEIGHT + 4. + i as f32 * LINE_HEIGHT
                )
            };
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(px, py),
                    Vector::new(32., 32.)
                ),
                Img(&img),
                Transform::scale(Vector::new(1., 1.)),
                100,
            );
        }

        Ok(())
    }

    pub fn step(&mut self) {
        let mut current = *self.pointer.last().unwrap();
        current -= 1;
        while current > 0 && self.items[current as usize] == None {
            current -= 1;
        }
        *self.pointer.last_mut().unwrap() = current;
    }

    pub fn as_rule(&self, id: usize) -> Option<Rule<BurgerItem>> {
        if self.name == None {
            return Option::None;
        }
        let mut production = vec![];
        if self.items.iter().all(|i| *i == None) {production.push(Token::Epsilon)}
        else {
            for i in &self.items {
                if *i == None {
                    // production.push(Token::Epsilon);
                } else if i.is_nonterm() {
                    production.push(Token::NonTerminal(i.to_str().to_owned()));
                } else {
                    production.push(Token::Terminal(*i))
                }
            }
            production.reverse();
        }

        Some(Rule {
            name: self.name.to_str().to_owned(),
            production,
            id,
        })

    }

}