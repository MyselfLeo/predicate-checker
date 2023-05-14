///! Small parser to convert a string into a predicate.



// Do not use this parser for another purpose than parsing predicates.
// A better parser would be the one used in Sloth, which is more general and more robust.
// cf. https://github.com/MyselfLeo/sloth

use std::{str::FromStr, any::type_name};

use num::Num;

use crate::{Predicate, Value};

const VALUE_OPS: [&str; 5] = ["==", ">", "<", ">=", "<="];
const PREDICATE_OPS: [&str; 3] = ["||", "&&", "!"];

const OPERATORS: [&str; 8] = ["==", ">", "<", ">=", "<=", "||", "&&", "!"];
const SEPARATORS: [&str; 2] = ["(", ")"];

#[derive(Debug, Clone, PartialEq)]
pub enum Token<T: Num> {
    Boolean(bool),
    Operator(String),
    Separator(String),
    Arg(String),
    Literal(T)
}



/// Convert a string into a Vec of tokens
pub fn parse<T: Num + FromStr>(txt: &str) -> Result<Vec<Token<T>>, String> {
    let txt_cleaned = txt.replace("(", " ( ").replace(")", " ) ");
    txt_cleaned.split(' ').filter(|x| !x.trim().is_empty()).map( |t|
        if t == "true" {Ok(Token::Boolean(true))}
        else if t == "false" {Ok(Token::Boolean(false))}
        else if OPERATORS.contains(&t) {Ok(Token::Operator(t.to_string()))}
        else if SEPARATORS.contains(&t) {Ok(Token::Separator(t.to_string()))}
        else if t.parse::<T>().is_ok() {
            match t.parse::<T>() {
                Ok(v) => Ok(Token::Literal(v)),
                Err(_) => return Err(format!("Unable to parse literal {t} into type {}", type_name::<T>()))
            }
        }
        else if t.chars().next().unwrap().is_alphabetic() {Ok(Token::Arg(t.to_string()))}
        else {Err(format!("Invalid token: {}", t))}
    ).collect()
}




/// Convert an infix vec of tokens into a postfix stream one
/// This function uses the Shunting-Yard algorithm
pub fn infix_to_postfix<T: Num + FromStr>(tokens: Vec<Token<T>>) -> Result<Vec<Token<T>>, String> {
    let mut res = vec![];
    let mut operator_stack = vec![];

    for t in tokens {
        match &t {
            Token::Operator(x) => {

                if VALUE_OPS.contains(&x.as_str()) {    // Value operators (<, ==, etc.) have a higher precidence than boolean operators (&&, ||, etc.)
                    while let Some(Token::Operator(x)) = operator_stack.last() {
                        if !VALUE_OPS.contains(&x.as_str()) {break;}
                        else {
                            res.push(operator_stack.pop().unwrap());
                        }
                    }
                }

                else if PREDICATE_OPS.contains(&x.as_str()) {
                    while let Some(Token::Operator(_)) = operator_stack.last() {
                        res.push(operator_stack.pop().unwrap());
                    }
                }

                operator_stack.push(t);
            },

            Token::Separator(s) => {
                if s == "(" {
                    operator_stack.push(t);
                }
                else if s == ")" {
                    while *operator_stack.last().unwrap() != Token::Separator("(".to_string()) {
                        res.push(operator_stack.pop().unwrap())
                    }
                    operator_stack.pop().unwrap();
                }
                else {return Err(format!("Invalid predicate string"))}
            },

            Token::Arg(_) => res.push(t),
            Token::Literal(_) => res.push(t),
            Token::Boolean(_) => res.push(t),
        }
    }    

    while !operator_stack.is_empty() {
        res.push(operator_stack.pop().unwrap())
    }

    Ok(res)
}





/// Create a predicate from a infix string for example `(x > 5) && (x < 10)
pub fn parse_predicate<T: Num + PartialOrd + FromStr>(txt: &str) -> Result<Predicate<T>, String> {
    let tokens = infix_to_postfix(parse(txt)?)?;

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
                    if value_stack.len() < 2 {return Err(format!("Invalid predicate string"))}

                    let v2 = value_stack.pop().unwrap();
                    let v1 = value_stack.pop().unwrap();

                    match op.as_str() {
                        //"==", ">", "<", ">=", "<="
                        "==" => predicate_stack.push(Predicate::Equal(v1, v2)),
                        ">" => predicate_stack.push(Predicate::GreaterThan(v1, v2)),
                        "<" => predicate_stack.push(Predicate::LowerThan(v1, v2)),
                        ">=" => predicate_stack.push(Predicate::GreaterEqual(v1, v2)),
                        "<=" => predicate_stack.push(Predicate::LowerEqual(v1, v2)),
                        _ => return Err(format!("Invalid predicate string"))
                    }
                }

                else if PREDICATE_OPS.contains(&op.as_str()) {
                    if predicate_stack.len() < 2 {return Err(format!("Invalid predicate string"))}

                    let p2 = predicate_stack.pop().unwrap();
                    let p1 = predicate_stack.pop().unwrap();

                    match op.as_str() {
                        //"||", "&&", "!"
                        "||" => predicate_stack.push(Predicate::Or(Box::new(p1), Box::new(p2))),
                        "&&" => predicate_stack.push(Predicate::And(Box::new(p1), Box::new(p2))),
                        "!" => predicate_stack.push(Predicate::Not(Box::new(p1))),
                        _ => return Err(format!("Invalid predicate string"))
                    }
                }
            },


            Token::Separator(_) => panic!("Unexpected separator"),
        }

    }

    // At this point there should be only one predicate in the stack
    match predicate_stack.pop() {
        Some(p) => Ok(p),
        None => Err(format!("Invalid predicate string"))
    }
}