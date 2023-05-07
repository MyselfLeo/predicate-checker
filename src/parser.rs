///! Small parser to convert a string into a predicate.



// Do not use this parser for another purpose than parsing predicates.
// A better parser would be the one used in Sloth, which is more general and more robust.
// cf. https://github.com/MyselfLeo/sloth

use crate::{Predicate, Value};

const VALUE_OPS: [&str; 5] = ["==", ">", "<", ">=", "<="];
const PREDICATE_OPS: [&str; 3] = ["||", "&&", "!"];

const OPERATORS: [&str; 8] = ["==", ">", "<", ">=", "<=", "||", "&&", "!"];
const SEPARATORS: [&str; 2] = ["(", ")"];

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Boolean(bool),
    Operator(String),
    Separator(String),
    Arg(String),
    Literal(f64)
}



/// Convert a string into a Vec of tokens
pub fn parse(txt: &String) -> Vec<Token> {
    let txt_cleaned = txt.replace("(", " ( ").replace(")", " ) ");
    txt_cleaned.split(' ').filter(|x| !x.trim().is_empty()).map( |t|
        if t == "true" {Token::Boolean(true)}
        else if t == "false" {Token::Boolean(false)}
        else if OPERATORS.contains(&t) {Token::Operator(t.to_string())}
        else if SEPARATORS.contains(&t) {Token::Separator(t.to_string())}
        else if t.parse::<f64>().is_ok() {Token::Literal(t.parse::<f64>().unwrap())}
        else if t.chars().next().unwrap().is_alphabetic() {Token::Arg(t.to_string())}
        else {panic!("Invalid token: {}", t)}
    ).collect()
}




/// Convert an infix vec of tokens into a postfix stream one
pub fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut res = vec![];
    let mut token_stack = vec![];

    for t in tokens {
        match &t {
            Token::Operator(_) => {
                token_stack.push(t);
            },

            Token::Separator(s) => {
                if s == "(" {
                    token_stack.push(t);
                }
                else if s == ")" {
                    while *token_stack.last().unwrap() != Token::Separator("(".to_string()) {
                        res.push(token_stack.pop().unwrap())
                    }
                    token_stack.pop().unwrap();
                }
                else {panic!("Unexpected separator: {s}")}
            },

            Token::Arg(_) => res.push(t),
            Token::Literal(_) => res.push(t),
            Token::Boolean(_) => res.push(t),
        }
    }    

    while !token_stack.is_empty() {
        res.push(token_stack.pop().unwrap())
    }

    res
}



/// Create a predicate from a infix string.
/// ex: "(x > 5) && (x < 10)"
pub fn parse_predicate(txt: &String) -> Predicate<f64> {
    let tokens = infix_to_postfix(parse(txt));

    let mut predicate_stack = vec![];
    let mut value_stack = vec![];

    for token in tokens {

        match token {

            Token::Boolean(true) => predicate_stack.push(Predicate::True),
            Token::Boolean(false) => predicate_stack.push(Predicate::False),
            Token::Arg(x) => value_stack.push(Value::Arg(x)),
            Token::Literal(l) => value_stack.push(Value::Literal(l)),


            Token::Operator(op) => {
                if VALUE_OPS.contains(&op.as_str()) {
                    if value_stack.len() < 2 {panic!("Unexpected operator: {op}")}

                    let v2 = value_stack.pop().unwrap();
                    let v1 = value_stack.pop().unwrap();

                    match op.as_str() {
                        //"==", ">", "<", ">=", "<="
                        "==" => predicate_stack.push(Predicate::Equal(v1, v2)),
                        ">" => predicate_stack.push(Predicate::GreaterThan(v1, v2)),
                        "<" => predicate_stack.push(Predicate::LowerThan(v1, v2)),
                        ">=" => predicate_stack.push(Predicate::GreaterEqual(v1, v2)),
                        "<=" => predicate_stack.push(Predicate::LowerEqual(v1, v2)),
                        _ => panic!("Unexpected operator: {op}")
                    }
                }

                else if PREDICATE_OPS.contains(&op.as_str()) {
                    if predicate_stack.len() < 2 {panic!("Unexpected operator: {op}")}

                    let p2 = predicate_stack.pop().unwrap();
                    let p1 = predicate_stack.pop().unwrap();

                    match op.as_str() {
                        //"||", "&&", "!"
                        "||" => predicate_stack.push(Predicate::Or(Box::new(p1), Box::new(p2))),
                        "&&" => predicate_stack.push(Predicate::And(Box::new(p1), Box::new(p2))),
                        "!" => predicate_stack.push(Predicate::Not(Box::new(p1))),
                        _ => panic!("Unexpected operator: {op}")
                    }
                }
            },


            Token::Separator(_) => panic!("Unexpected separator"),
        }

    }

    // At this point there should be only one predicate in the stack
    if predicate_stack.len() != 1 {panic!("Invalid predicate")}
    predicate_stack.pop().unwrap()
}