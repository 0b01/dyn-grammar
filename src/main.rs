#[derive(Clone, Debug)]
struct Rule {
    name: String,
    production: Vec<Token>,
}

#[derive(Clone, Debug)]
struct Grammar {
    rules: Vec<Rule>,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Terminal(String),
    Epsilon,
    NonTerminal(String),
}

use self::Token::*;

fn main() {
}

impl Grammar {

    fn parse(&self, name: &str, sent: &mut Vec<Token>) {
        let mut sent = sent;

        let first_set = self.first_set(name);
        let rule = first_set.iter().find(|prod|prod.0 == sent[0]);
        let mut rule_prod = match rule {
            Some(i) => i.1.production.clone(),
            None => // match epsilon
                match first_set.iter().find(|prod| prod.0 == Epsilon) {
                    None => panic!(""),
                    Some((_,i)) => i.production.clone(),
                }
        };
        self.match_rule(&mut sent, &mut rule_prod);
    }

    fn match_rule(&self, sent: &mut Vec<Token>, rule: &mut Vec<Token>) {
        while let Some(t) = rule.get(0) {
            let t = t.clone();
            println!("{:?}", t);
            rule.remove(0);
            match t {
                Epsilon => {
                    println!("epsilon", );
                }
                Terminal(_) => {
                    if let Some(sent_tok) = sent.get(0) {
                        if sent_tok.clone() == t {
                            sent.remove(0);
                        }
                    } else {
                        panic!();
                    }
                }
                NonTerminal(s) => self.parse(&s, sent),
            }
        }
    }

    fn first_set(&self, name: &str) -> Vec<(Token, Rule)> {
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

        let mut sent = vec![
            Terminal("a".to_owned()),
            Terminal("a".to_owned()),
            Terminal("b".to_owned()),
            Terminal("b".to_owned()),
        ];

        g.parse("S", &mut sent);
        assert!(sent.is_empty());
    }
}