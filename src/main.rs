use implicity::parser;

fn main() {
    let a: implicity::Predicate<f64> = parser::parse_predicate("(x > 2) && (y == 4)");
    let b = parser::parse_predicate("(x > 0) && (y > 2)");

    println!("Domain of a for x: {:?}     for y: {:?}", a.get_domain("x"), a.get_domain("y"));
    println!("Domain of b for x: {:?}     for y: {:?}", b.get_domain("x"), b.get_domain("y"));

    println!("{}", a.fits(&b));
}