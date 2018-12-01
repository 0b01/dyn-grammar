use crate::prelude::*;
use crate::grammar::parser::Token;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum BurgerItem {
    TopBun,
    BottomBun,
    Chicken,
    Beef,
    Lettuce,
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
    pub fn to_anim_str(&self) -> &'static str {
        match &self {
            BurgerItem::Lettuce => "place_lettuce",
            BurgerItem::TopBun => "place_top_bun",
            BurgerItem::BottomBun => "place_bottom_bun",
            BurgerItem::Chicken => "place_chicken",
            BurgerItem::Beef => "place_beef",
            BurgerItem::Cheese => "place_cheese",
            BurgerItem::Fish => "place_fish",
            BurgerItem::Onion => "place_onion",
            BurgerItem::Ketchup => "squeeze_ketchup",
            BurgerItem::Mayo => "squeeze_mayo",
            BurgerItem::BBQ => "squeeze_bbq",

            BurgerItem::NonTermA => "a",
            BurgerItem::NonTermB => "b",
            BurgerItem::NonTermC => "c",
            BurgerItem::NonTermS => "s",
            BurgerItem::NonTermEpsilon => "e",
        }
    }
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
            BurgerItem::Lettuce => "lettuce",

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


#[derive(Clone)]
pub struct Burger {
    toks: Vec<Token<BurgerItem>>,
}

impl Burger {
    pub fn new() -> Self {
        use self::BurgerItem::*;
        let mut toks = vec![];
        toks.push(Token::Terminal(BottomBun));
        toks.push(Token::Terminal(Beef));
        toks.push(Token::Terminal(Cheese));
        toks.push(Token::Terminal(Lettuce));
        toks.push(Token::Terminal(Ketchup));
        toks.push(Token::Terminal(TopBun));

        Self {
            toks,
        }
    }

    /// draw a static burger
    pub fn draw(&self,
        window: &mut Window,
        ingredients: &mut Asset<Ingredients>,
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
                    ingredients.execute(|ing| {

                        let img = ing.get(item).unwrap();
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new( init_x, init_y - i * dy ),
                                Vector::new( 32., 32. )
                            ),
                            Img(&img),
                            Transform::scale(Vector::new(3., 3.)),
                            i as u32,
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

pub struct BurgerAnimSeq {
    current_t: f64,
    burger: Burger,
    idx: usize,
    static_idx: usize,
    drawing: Option<BurgerItem>,
}

impl BurgerAnimSeq {
    pub fn new(burger: Burger) -> Self {
        // let played = vec![false; burger.toks.len()];
        Self {
            current_t: 0.,
            burger,
            idx: 0,
            static_idx: 0,
            drawing: None,
        }
    }

    pub fn draw(
        &mut self,
        window: &mut Window,
        ingredients: &mut Asset<Ingredients>,
        ing_anim: &mut Asset<IngredientAnimations>,
    ) -> Result<()> {

        let dy = 13.;
        ing_anim.execute(|ing_anim| {
            ing_anim.draw(window, 565., 190. - self.idx as f32 * dy, 2.)?;
            Ok(())
        })?;

        if self.drawing.is_some() {
            let anim = self.drawing.as_ref().unwrap().to_anim_str();
            ing_anim.execute(|ing_anim| {
                let played = ing_anim.get_mut(anim).unwrap().played;
                if played {
                    println!("Done playing", );
                    self.static_idx += 1;
                    self.drawing = None;
                }
                Ok(())
            })?;
        }
        self.burger.draw(window, ingredients, Some(self.static_idx))?;

        Ok(())
    }

    pub fn step(
        &mut self,
        ing_anim: &mut Asset<IngredientAnimations>,
    ) -> Result<()> {
        if let Token::Terminal(itm) = &self.burger.toks[self.idx] {
            self.drawing = Some(itm.clone());

            let anim = itm.to_anim_str();
            ing_anim.execute(|ing_anim| {
                ing_anim.get_mut(anim).unwrap().play()?;
                Ok(())
            })?;
        }


        self.idx += 1;
        Ok(())
    }


}