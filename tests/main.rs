use predicatechecker::{Predicate, Implication};

fn main() {
    let a: Predicate<i64> = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap();
    let b: Predicate<i64> = Predicate::from("(x > 0) && (y > 2)").unwrap();

    assert_eq!(a.implies(&b), Implication::Total);
}