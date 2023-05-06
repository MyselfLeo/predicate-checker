pub mod predicate;
mod domain;

fn main() {
    
    /*
    let p1 = predicate::Predicate::LowerThan(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(6)
    );

    let p2 = predicate::Predicate::GreaterEqual(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(2)
    );

    let p3 = predicate::Predicate::And(Box::new(p1.clone()), Box::new(p2.clone()));

    println!("{:?}", p1.get_domain());
    println!("{:?}", p2.get_domain());
    println!("{:?}", p3.get_domain());
     */

    let p1 = predicate::Predicate::LowerThan(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(6)
    );
    let p2 = predicate::Predicate::GreaterEqual(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(7)
    );

    let p3 = predicate::Predicate::Or(Box::new(p1.clone()), Box::new(p2.clone()));

    let p4 = predicate::Predicate::LowerThan(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(4)
    );

    println!("{:?}", p3.fits(&p4));
}
