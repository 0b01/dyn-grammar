use crate::grammar::Token;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Ingredient {
    TopBun,
    BottomBun,
    Patty,
    Sriracha,
    Ketchup,
    IDK,
}

trait Burger {
    fn name() -> &'static str;
    fn description() -> &'static str;
}

pub enum BurgerType {
    ElMcGangbang,
}

struct ElMcGangbang {
    examples: Vec<Token<Ingredient>>,
}

impl Burger for ElMcGangbang {

    fn name() -> &'static str {
        "El McGangbang"
    }

    fn description() -> &'static str {
        "The McChicken is placed directly inside the cheeseburger."
    }

}