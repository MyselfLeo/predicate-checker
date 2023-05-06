///! Small parser to convert a string into a predicate.



// Do not use this parser for another purpose than parsing predicates.
// A better parser would be the one used in Sloth, which is more general and more robust.
// cf. https://github.com/MyselfLeo/sloth

use crate::Predicate;


const OPERATORS: [&str; 8] = ["==", ">", "<", ">=", "<=", "||", "&&", "!"];
const SEPARATORS: [&str; 2] = ["(", ")"];

#[derive(Debug)]
pub enum Token {
    Boolean(bool),
    Operator(String),
    Separator(String),
    Arg(String),
    Literal(f64)
}



/// Convert a string into a Vec of tokens
fn parse(txt: String) -> Vec<Token> {
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




pub fn parse_predicate(txt: String) -> Predicate<f64> {
    let tokens = parse(txt);
    
    let mut token_buffer = vec![];
    let mut predicate_buffer = vec![];

    

    for token in tokens {

        token_buffer.push(token);



        if token_buffer.last().unwrap()


    }

    
    
    todo!()
}