use crate::grammar::*;
use self::Token::*;

macro_rules! sentence {
    ($($i: ident),*) => {
        {
            let mut v = vec![];
            $(
                v.push(Terminal(stringify!($i)));
            )*
            v
        }
    };
}


#[test]
fn test_parse_simple_grammar() {
    // Grammar
    // S -> a S b .
    // S -> .

    let mut g = Grammar::new(
        "S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    Terminal("a"),
                    NonTerminal("S".to_owned()),
                    Terminal("b"),
                ]
            },
            Rule {
                name: "S".to_owned(),
                id: 1,
                production: vec![
                    Epsilon,
                ]
            }
        ]
    );
    g.build().unwrap();

    // assert!(g.parse(sentence!(a,b)).is_ok());
    // assert!(g.parse(sentence!(a,a,b,b)).is_ok());

    // assert!(g.parse(sentence!(a,a)).is_err());
    // assert!(g.parse(sentence!(a,a,b)).is_err());
    // assert!(g.parse(sentence!(a,a,b,b,b)).is_err());

    let ret = g.parse(sentence!(a,b));
    println!("{:#?}", ret);
}

#[test]
fn test_first_set_clash() {
    let mut g = Grammar::new(
        "S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    Terminal("a"),
                    Terminal("a"),
                    NonTerminal("S".to_owned()),
                ]
            },
            Rule {
                name: "S".to_owned(),
                id: 1,
                production: vec![
                    Terminal("a"),
                    Epsilon,
                ]
            }
        ]
    );

    assert!(g.build().is_err());
}

#[test]
fn test_ab() {
    // Grammar:
    // S -> a A b B.
    // A -> a A .
    // B -> b B .
    // A -> .
    // B -> .

    let mut g = Grammar::new("S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    Terminal("a"),
                    NonTerminal("A".to_owned()),
                    Terminal("b"),
                    NonTerminal("B".to_owned()),
                ]
            },
            Rule {
                name: "A".to_owned(),
                id: 1,
                production: vec![
                    Terminal("a"),
                    NonTerminal("A".to_owned()),
                ]
            },
            Rule {
                name: "B".to_owned(),
                id: 2,
                production: vec![
                    Terminal("b"),
                    NonTerminal("B".to_owned()),
                ]
            },
            Rule {
                name: "A".to_owned(),
                id: 3,
                production: vec![
                    Epsilon,
                ]
            },
            Rule {
                name: "B".to_owned(),
                id: 4,
                production: vec![
                    Epsilon,
                ]
            },
        ]
    );

    g.build().unwrap();
    g.parse(sentence!(a, b)).unwrap();

    assert!(g.parse(sentence!(a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, a, b)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b, b, b, b)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b, b, b, b)).is_ok());


    assert!(g.parse(sentence!(a, a)).is_err());

}

#[test]
fn test_abc() {
    // Grammar:
    // S -> a A .
    // A -> a A .
    // A -> b B .
    // B -> b B .
    // B -> c C .
    // C -> c C .
    // C -> .

    // Example:
    // TODO:

    let mut g = Grammar::new(
        "S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    Terminal("a"),
                    NonTerminal("A".to_owned()),
                ]
            },
            Rule {
                name: "A".to_owned(),
                id: 1,
                production: vec![
                    Terminal("a"),
                    NonTerminal("A".to_owned()),
                ]
            },
            Rule {
                name: "A".to_owned(),
                id: 2,
                production: vec![
                    NonTerminal("B".to_owned()),
                ]
            },
            Rule {
                name: "B".to_owned(),
                id: 3,
                production: vec![
                    Terminal("c"),
                ]
            },
            Rule {
                name: "B".to_owned(),
                id: 4,
                production: vec![
                    Terminal("b"),
                    NonTerminal("B".to_owned()),
                ]
            },
        ]
    );

    g.build().unwrap();

    assert!(g.parse(sentence!(a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, b, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, b, b, b, b, b, b, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, a, a, c)).is_ok());
    assert!(g.parse(sentence!(a, a, a, a, a, a, a, b, c)).is_ok());

    assert!(g.parse(sentence!(a, a)).is_err());
    assert!(g.parse(sentence!(a, b)).is_err());
    assert!(g.parse(sentence!(b, b)).is_err());
    assert!(g.parse(sentence!(b, c)).is_err());
    assert!(g.parse(sentence!(b, a)).is_err());
    assert!(g.parse(sentence!(c, b, a)).is_err());
    assert!(g.parse(sentence!(c, a)).is_err());
}

#[test]
fn test_stackoverflow() {
    let mut g = Grammar::new(
        "S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    NonTerminal("S".to_owned()),
                    Terminal("a"),
                    Terminal("b"),
                ]
            },
            Rule {
                name: "S".to_owned(),
                id: 1,
                production: vec![
                    Epsilon,
                ]
            }
        ]
    );
    assert!(g.build().is_err());
}

#[test]
fn test_stackoverflow_other_case() {
    let mut g = Grammar::new(
        "S".to_owned(),
        vec![
            Rule {
                name: "S".to_owned(),
                id: 0,
                production: vec![
                    Terminal("a"),
                    NonTerminal("S".to_owned()),
                    Terminal("b"),
                ]
            },
            Rule {
                name: "S".to_owned(),
                id: 1,
                production: vec![
                    NonTerminal("S".to_owned()),
                    Terminal("a"),
                    Terminal("a"),
                    Epsilon,
                ]
            }
        ]
    );
    assert!(g.build().is_err());
}
