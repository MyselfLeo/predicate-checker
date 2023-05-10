use predicatechecker::Predicate;

fn main() {
    //let a = Predicate::from("x > 2 && y == 4").unwrap();
    //let b = Predicate::from("x > 3 && y > 2").unwrap();

    let a = Predicate::from("x > 0").unwrap();
    let b = Predicate::from("x > -3").unwrap();

    println!("{:?}", a.implies(&b));

    //let b = Predicate::from("(x > 0) && (y > 2)").unwrap();
    //println!("{}", a.implies(&b));
}