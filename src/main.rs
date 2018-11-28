use self::Token::*;
use std::fmt::Debug;

#[derive(Clone, Debug)]
struct Rule<T: Debug + Clone + PartialEq> {
    name: String,
    production: Vec<Token<T>>,
}

#[derive(Clone, Debug)]
struct Grammar<T: Debug + Clone + PartialEq> {
    start: String,
    rules: Vec<Rule<T>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Token<T: Debug + Clone + PartialEq> {
    Terminal(T),
    Epsilon,
    NonTerminal(String),
}

macro_rules! sentence {
    ($($i: ident),*) => {
        {
            let mut v = vec![];
            $(
                v.push(Terminal(stringify!($i).to_owned()));
            )*
            v
        }
    };
}

fn main() {
}

impl<T: Debug + PartialEq + Clone> Grammar<T> {

    fn parse(&self, sent: Vec<Token<T>>) -> Result<(), String> {
        let mut sent = sent;
        self.parse_aux(&self.start, &mut sent)?;
        if sent.is_empty() {
            return Ok(())
        } else {
            return Err("Incomplete parse".to_owned())
        }
    }

    fn parse_aux(&self, name: &str, sent: &mut Vec<Token<T>>) -> Result<(), String> {
        let mut sent = sent;
        let first_set = self.first_set(name);
        println!("Parsing rule {} with {:?}", name, sent);
        if sent.is_empty() { return Err("Does not parse".to_owned()); }
        let rule = first_set.iter().find(|prod|prod.0 == sent[0]);
        let mut prod = match rule {
            Some(i) => i.1.production.clone(),
            None => // match epsilon
                match first_set.iter().find(|prod| prod.0 == Epsilon) {
                    None => { return Err("Incomplete".to_owned()); }
                    Some((_,i)) => i.production.clone(),
                }
        };
        println!("Found: {:?}", prod);
        self.match_rule(&mut sent, &mut prod)
    }

    fn match_rule(&self, sent: &mut Vec<Token<T>>, rule: &mut Vec<Token<T>>) -> Result<(), String> {
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
                        return Err("2".to_owned())
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
        let g = Grammar {
            start: "S".to_owned(),
            rules: vec![
                Rule {
                    name: "S".to_owned(),
                    production: vec![
                        Terminal("a".to_owned()),
                        NonTerminal("S".to_owned()),
                        Terminal("b".to_owned()),
                    ]
                },
                Rule {
                    name: "S".to_owned(),
                    production: vec![
                        Epsilon,
                    ]
                }
            ]
        };

        assert!(g.parse(sentence!(a,b)).is_ok());
        assert!(g.parse(sentence!(a,a,b,b)).is_ok());

        assert!(g.parse(sentence!(a,a)).is_err());
        assert!(g.parse(sentence!(a,a,b)).is_err());
        assert!(g.parse(sentence!(a,a,b,b,b)).is_err());
    }

    // #[test]
    // fn test_complex1() {
    //     let g = Grammar {
    //         start: "S".to_owned(),
    //         rules: vec![
    //             Rule {
    //                 name: "S".to_owned(),
    //                 production: vec![
    //                     Terminal("a". to_owned()),
    //                     Terminal("a". to_owned()),
    //                     NonTerminal("S". to_owned()),
    //                 ]
    //             },
    //             Rule {
    //                 name: "S".to_owned(),
    //                 production: vec![
    //                     Terminal("a".to_owned()),
    //                     Epsilon,
    //                 ]
    //             }
    //         ]
    //     };

    //     g.parse(sentence!(a,a,a)).unwrap();
    // }
}