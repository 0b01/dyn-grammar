use crate::prelude::*;
use crate::grammar::parser::Token;
use super::BurgerItem;

#[derive(Clone)]
pub struct Burger {
    pub toks: Vec<Token<BurgerItem>>,
}

impl Burger {
    pub fn new() -> Self {
        use self::BurgerItem::*;
        let mut toks = vec![];
        toks.push(Token::Terminal(BottomBun));
        toks.push(Token::Terminal(BottomBun));
        toks.push(Token::Terminal(TopBun));
        toks.push(Token::Terminal(TopBun));

        Self {
            toks,
        }
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

    pub fn as_token(&self) -> Vec<Token<BurgerItem>> {
        return self.toks.clone();
    }

}
