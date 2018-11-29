use self::Token::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug)]
struct Rule<T: Debug + Clone + PartialEq + Hash + Eq> {
    name: String,
    production: Vec<Token<T>>,
}

#[derive(Clone, Debug)]
struct Grammar<T: Debug + Clone + PartialEq + Hash + Eq> {
    start: String,
    rules: Vec<Rule<T>>,
    first_set: Option<HashMap<String, Vec<(Token<T>, Rule<T>)>>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Token<T: Debug + Clone + PartialEq + Hash + Eq> {
    Terminal(T),
    Epsilon,
    NonTerminal(String),
}

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


impl<T: Debug + Clone + PartialEq + Hash + Eq> Grammar<T> {

    fn new(start: String, rules: Vec<Rule<T>>) -> Self {
        let first_set = None;
        Self {
            start,
            rules,
            first_set
        }
    }

    pub fn build(&mut self) -> Result<(), &'static str> {
        let mut temp = HashMap::new();
        for rule in &self.rules {
            let name = rule.name.clone();
            let first_set_for_rule = self.first_set(&name);
            // check first set clashes
            let mut temp_dedup = HashSet::new();
            for (tok,_) in &first_set_for_rule {
                if temp_dedup.contains(tok) {
                    return Err("First/First clash");
                }
                temp_dedup.insert(tok.clone());
            }
            temp.insert(name, first_set_for_rule);
        }

        self.first_set = Some(temp);

        Ok(())
    }

    fn parse(&self, sent: Vec<Token<T>>) -> Result<(), &'static str> {
        let mut sent = sent;
        self.parse_aux(&self.start, &mut sent)?;
        if sent.is_empty() {
            return Ok(())
        } else {
            return Err("Incomplete parse")
        }
    }

    fn parse_aux(&self, name: &str, sent: &mut Vec<Token<T>>) -> Result<(), &'static str> {
        let mut sent = sent;
        let first_set = self.first_set(name);
        println!("Parsing rule {} with {:?}", name, sent);
        if sent.is_empty() { return Err("Does not parse"); }
        let rule = first_set.iter().find(|prod|prod.0 == sent[0]);
        let mut prod = match rule {
            Some(i) => i.1.production.clone(),
            None => // match epsilon
                match first_set.iter().find(|prod| prod.0 == Epsilon) {
                    None => { return Err("Incomplete"); }
                    Some((_,i)) => i.production.clone(),
                }
        };
        println!("Found: {:?}", prod);
        self.match_rule(&mut sent, &mut prod)
    }

    fn match_rule(&self, sent: &mut Vec<Token<T>>, rule: &mut Vec<Token<T>>) -> Result<(), &'static str> {
        while let Some(t) = rule.get(0) {
            let t = t.clone();
            rule.remove(0);
            match t {
                Epsilon => {
                    println!("Matching Epsilon");
                }
                Terminal(ref term) => {
                    println!("Matching {:?}", term);
                    if let Some(sent_tok) = sent.get(0) {
                        if sent_tok.clone() == t {
                            sent.remove(0);
                        }
                    } else {
                        return Err("2")
                    }
                }
                NonTerminal(s) => self.parse_aux(&s, sent)?
            }
        }

        Ok(())
    }

    fn first_set(&self, name: &str) -> Vec<(Token<T>, Rule<T>)> {
        let mut ret = vec![];
        for rule in &self.rules {
            if rule.name != name {
                continue;
            }
            // this is incorrect...
            let first_tok = rule.production[0].clone();
            let first = match first_tok {
                Terminal(_) | Epsilon => vec![(first_tok, rule.clone())],
                NonTerminal(s) => self.first_set(&s),
            };
            ret.extend(first);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                    production: vec![
                        Terminal("a"),
                        NonTerminal("S".to_owned()),
                        Terminal("b"),
                    ]
                },
                Rule {
                    name: "S".to_owned(),
                    production: vec![
                        Epsilon,
                    ]
                }
            ]
        );
        g.build().unwrap();

        assert!(g.parse(sentence!(a,b)).is_ok());
        assert!(g.parse(sentence!(a,a,b,b)).is_ok());

        assert!(g.parse(sentence!(a,a)).is_err());
        assert!(g.parse(sentence!(a,a,b)).is_err());
        assert!(g.parse(sentence!(a,a,b,b,b)).is_err());
    }

    #[test]
    fn test_first_set_clash() {
        let mut g = Grammar::new(
            "S".to_owned(),
            vec![
                Rule {
                    name: "S".to_owned(),
                    production: vec![
                        Terminal("a"),
                        Terminal("a"),
                        NonTerminal("S".to_owned()),
                    ]
                },
                Rule {
                    name: "S".to_owned(),
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
    fn test_abc() {

        // Grammar:
        // S -> a A .
        // A -> a A .
        // A -> B .
        // B -> c .
        // B -> b B .

        // Example:
        // a c
        // a a c
        // a b c
        // a a a c
        // a a b c
        // a b b c
        // a a a a c
        // a a a b c
        // a a b b c
        // a b b b c
        // a a a a a c
        // a a a a b c
        // a a a b b c
        // a a b b b c
        // a b b b b c
        // a a a a a a c
        // a a a a a b c
        // a a a a b b c
        // a a a b b b c
        // a a b b b b c
        // a b b b b b c
        // a a a a a a a c
        // a a a a a a b c
        // a a a a a b b c
        // a a a a b b b c
        // a a a b b b b c
        // a a b b b b b c
        // a b b b b b b c
        // a a a a a a a a c
        // a a a a a a a b c

        let mut g = Grammar::new(
            "S".to_owned(),
            vec![
                Rule {
                    name: "S".to_owned(),
                    production: vec![
                        Terminal("a"),
                        NonTerminal("A".to_owned()),
                    ]
                },
                Rule {
                    name: "A".to_owned(),
                    production: vec![
                        Terminal("a"),
                        NonTerminal("A".to_owned()),
                    ]
                },
                Rule {
                    name: "A".to_owned(),
                    production: vec![
                        NonTerminal("B".to_owned()),
                    ]
                },
                Rule {
                    name: "B".to_owned(),
                    production: vec![
                        Terminal("c"),
                    ]
                },
                Rule {
                    name: "B".to_owned(),
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

}