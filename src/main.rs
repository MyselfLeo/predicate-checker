use implicity::parser;

fn main() {
    let a: implicity::Predicate<f64> = parser::parse_predicate("(x > 2) && (y == 4) && (z < 10)");
    let b = parser::parse_predicate("(x > 0) && (y > 2)");

    println!("{}", a.fits(&b));
}