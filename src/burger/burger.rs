use crate::prelude::*;
use crate::grammar::parser::Token;
use super::BurgerItem;

#[derive(Clone)]
pub struct Burger {
    pub toks: Vec<Token<BurgerItem>>,
}

macro_rules! burger {
    ($($i: ident),*) => {
        {
            let mut b = Burger::new();
            $(
                b.push(Token::Terminal(BurgerItem::$i));
            )*
            b.reverse();
            b
        }
    };
}

impl Burger {
    pub fn new() -> Self {
        let mut toks = vec![];
        Self {
            toks,
        }
    }

    pub fn reverse(&mut self) {
        self.toks.reverse();
    }
    pub fn push(&mut self, i: Token<BurgerItem>) {
        self.toks.push(i);
    }

    /// draw a static burger
    pub fn draw(&self,
        window: &mut Window,
        Sprites: &mut Sprites,
        to: Option<usize>,
    ) -> Result<()> {
        let to = to.unwrap_or(self.toks.len());
        let init_x = 595.; let init_y = 230.;
        let dy = 13.;
        let mut i = 0.;
        for tok in &self.toks[..to] {
            match tok {
                Token::Epsilon | Token::NonTerminal(_) => {continue; }
                Token::Terminal(burger_item) => {
                    let item = burger_item.to_str();
                    let img = Sprites.get_img(item).unwrap();
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new( init_x, init_y - i * dy ),
                            Vector::new( 32., 32. )
                        ),
                        Img(&img),
                        Transform::scale(Vector::new(3., 3.)),
                        10 + i as u32,
                    );
                }
            }
            i += 1.;
        }

        Ok(())
    }

    /// draw a static burger in order area
    pub fn draw_order(&self,
        window: &mut Window,
        Sprites: &mut Sprites,
    ) -> Result<()> {
        let init_x = 350.; let init_y = 320.;
        let _end_x = 300.; let _end_y = 363.;
        let dy = (360. - 70.) / self.toks.len() as f32;
        let mut i = 0.;
        for tok in &self.toks {
            match tok {
                Token::Epsilon | Token::NonTerminal(_) => {continue; }
                Token::Terminal(burger_item) => {
                    let item = burger_item.to_str();
                    let img = Sprites.get_img(item).unwrap();
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new( init_x, init_y - i * dy ),
                            Vector::new( 32., 32. )
                        ),
                        Img(&img),
                        Transform::scale(Vector::new(3., 3.)),
                        10 + i as u32,
                    );
                }
            }
            i += 1.;
        }

        Ok(())
    }

    pub fn as_token(&self) -> Vec<Token<BurgerItem>> {
        return self.toks.clone();
    }

}
