use implicity::{Predicate, Value};
use implicity::parser;

fn main() {
    
    /*
    let p1 = Predicate::LowerThan(
        Value::Arg("x".to_string()),
        Value::Literal(6)
    );

    let p2 = Predicate::GreaterEqual(
        Value::Arg("x".to_string()),
        Value::Literal(2)
    );

    let p3 = Predicate::And(Box::new(p1.clone()), Box::new(p2.clone()));

    println!("{:?}", p1.get_domain());
    println!("{:?}", p2.get_domain());
    println!("{:?}", p3.get_domain());
     */





    /*
    let p1 = Predicate::Or(
        Box::new(
            Predicate::LowerThan(
                Value::Arg("x".to_string()),
                Value::Literal(6)
            )
        ),
        Box::new(
            Predicate::GreaterEqual(
                Value::Arg("x".to_string()),
                Value::Literal(7)
            )
        )
    );

    let p2 = Predicate::Or(
        Box::new(
            Predicate::LowerThan(
                Value::Arg("x".to_string()),
                Value::Literal(6)
            )
        ),
        Box::new(
            Predicate::GreaterEqual(
                Value::Arg("x".to_string()),
                Value::Literal(8)
            )
        )
    );

    println!("{:?}", p2.fits(&p1));

     */


    println!("{:?}", parser::parse("(x > 5) && (x < 3)".to_string()));
}
