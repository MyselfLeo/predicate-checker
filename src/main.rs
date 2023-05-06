pub mod predicate;
mod domain;

fn main() {
    
    let p1 = predicate::Predicate::GreaterThan(
        predicate::Value::Literal(4),
        predicate::Value::Arg("x".to_string())
    );

    let p2 = predicate::Predicate::LowerThan(
        predicate::Value::Arg("x".to_string()),
        predicate::Value::Literal(10)
    );

    let p3 = predicate::Predicate::And(
        Box::new(p1),
        Box::new(p2)
    );


    println!("{:?}", p3.get_domain());
}
