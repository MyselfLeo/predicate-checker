use implicity::parser;

fn main() {
    let p1 = parser::parse_predicate("(x >= 3) && (x <= 3)");
    println!("Predicate: {:?}", p1);
    println!("Domain: {:?}", p1.get_domain());
}
