use crate::prelude::*;
use crate::grammar::parser::Token;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum BurgerItem {
    TopBun,
    BottomBun,
    Chicken,
    Beef,
    Cheese,
    Fish,
    Onion,
    NonTermS,
    NonTermA,
    NonTermB,
    NonTermC,
    NonTermEpsilon,
    Ketchup,
    Mayo,
    BBQ,
}

impl BurgerItem {
    pub fn to_str(&self) -> &'static str {
        match &self {
            BurgerItem::TopBun => "top_bun",
            BurgerItem::BottomBun => "bottom_bun",
            BurgerItem::Chicken => "chicken",
            BurgerItem::Beef => "beef_patty",
            BurgerItem::Cheese => "cheese",
            BurgerItem::Fish => "fish",
            BurgerItem::Onion => "onion",
            BurgerItem::Ketchup => "ketchup",
            BurgerItem::Mayo => "mayo",
            BurgerItem::BBQ => "bbq",

            BurgerItem::NonTermA => "a",
            BurgerItem::NonTermB => "b",
            BurgerItem::NonTermC => "c",
            BurgerItem::NonTermS => "s",
            BurgerItem::NonTermEpsilon => "e",
        }
    }
}

trait Level {
    fn name() -> &'static str;
    fn description() -> &'static str;
}

pub enum Levels {
    /// even number of patties
    DoubleCheeseburger,
    /// a^n b a^n
    WcGangbang,
    /// a^n b^m c^l
    LandSeaAndAir,
}

/// S -> top cheese beef cheese beef N bottom .
/// N -> cheese beef cheese beef N .
/// N -> .
struct DoubleCheeseburger {
    examples: Vec<Token<BurgerItem>>,
}

impl DoubleCheeseburger {
    fn name() -> &'static str {
        "DoubleCheeseburger"
    }

    fn description() -> &'static str {
        "Even number of patties"
    }
}

/// S -> top N bottom .
/// N -> beef S beef .
/// N -> chicken .
/// S -> .
struct WcGangbang {
    examples: Vec<Token<BurgerItem>>,
}

impl Level for WcGangbang {

    fn name() -> &'static str {
        "El McGangbang"
    }

    fn description() -> &'static str {
        "The WcChicken is placed directly inside The Big Wack."
    }

}

///
struct LandSeaAndAir {
    examples: Vec<Token<BurgerItem>>,
}

impl Level for LandSeaAndAir {
    fn name() -> &'static str {
        "Land, Sea, and Air"
    }
    fn description() -> &'static str {
        "Beef, fish of filet, chicken. In that order"
    }
}


pub struct Burger {
    toks: Vec<Token<BurgerItem>>,
}

impl Burger {
    pub fn new() -> Self {
        use self::BurgerItem::*;
        let mut toks = vec![];
        toks.push(Token::Terminal(TopBun));
        toks.push(Token::Terminal(Cheese));
        toks.push(Token::Terminal(Beef));
        toks.push(Token::Terminal(BottomBun));

        Self {
            toks,
        }
    }

    pub fn draw(&self, window: &mut Window, ingredients: &mut Asset<Ingredients>) -> Result<()> {
        // x: 575., y: 170.
        let init_x = 575.;
        let init_y = 170.;
        let dy = 23.;
        let mut i = 0.;
        for tok in &self.toks {
            match tok {
                Token::Epsilon | Token::NonTerminal(_) => {continue; }
                Token::Terminal(burger_item) => {
                    let item = burger_item.to_str();
                    ingredients.execute(|ing| {

                        let img = ing.get(item).unwrap();
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new( init_x, init_y + i * dy ),
                                Vector::new( 32., 32. )
                            ),
                            Img(&img),
                            Transform::scale(Vector::new(3., 3.)),
                            100 - i as u32,
                        );

                        Ok(())
                    })?;

                }
            }


            i += 1.;
        }

        Ok(())
    }
}