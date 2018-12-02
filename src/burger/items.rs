#[derive(Clone, Hash, Eq, PartialEq, Debug, Copy)]
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
    None,
}

impl BurgerItem {
    pub fn is_nonterm(&self) -> bool {
        use self::BurgerItem::*;
        match &self {
            NonTermS | NonTermA | NonTermB | NonTermC | NonTermEpsilon =>
                true,
            _ => false,
        }
    }
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

            BurgerItem::NonTermA => "nontermA",
            BurgerItem::NonTermB => "nontermB",
            BurgerItem::NonTermC => "nontermC",
            BurgerItem::NonTermS => "nontermS",
            BurgerItem::NonTermEpsilon => "nontermE",
            BurgerItem::None => "",
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

            BurgerItem::NonTermA => "nontermA",
            BurgerItem::NonTermB => "nontermB",
            BurgerItem::NonTermC => "nontermC",
            BurgerItem::NonTermS => "nontermS",
            BurgerItem::NonTermEpsilon => "nonterme",

            BurgerItem::None => "",
        }
    }
}
