use predicatechecker::Predicate;

fn main() {
    let a = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap();
    let b = Predicate::from("(x > 0) && (y > 2)").unwrap();

    println!("{}", a.fits(&b));
}