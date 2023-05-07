use implicity::parser;

fn main() {
    let p1 = parser::parse_predicate("((x >= 3) && (x <= 3)) || (y == 3)");
    let p2 = parser::parse_predicate("(y == 2) || (x == 3)");
    println!("{}", p1.fits(&p2));
}