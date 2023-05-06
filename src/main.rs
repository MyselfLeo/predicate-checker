use crate::{predicate::Predicate, domain::Domain};

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


    let p1 = predicate::Predicate::Or(
        Box::new(
            predicate::Predicate::LowerThan(
                predicate::Value::Arg("x".to_string()),
                predicate::Value::Literal(6)
            )
        ),
        Box::new(
            predicate::Predicate::GreaterEqual(
                predicate::Value::Arg("x".to_string()),
                predicate::Value::Literal(7)
            )
        )
    );

    let p2 = predicate::Predicate::Or(
        Box::new(
            predicate::Predicate::LowerThan(
                predicate::Value::Arg("x".to_string()),
                predicate::Value::Literal(5)
            )
        ),
        Box::new(
            predicate::Predicate::GreaterEqual(
                predicate::Value::Arg("x".to_string()),
                predicate::Value::Literal(8)
            )
        )
    );

    println!("{:?}", p2.fits(&p1));
}
