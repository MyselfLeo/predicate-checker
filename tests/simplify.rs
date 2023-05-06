use implicity::{Predicate, Value};

type F = f64;

#[test]
fn not() {
    let true_pred = Box::new(Predicate::<F>::True);
    let false_pred = Box::new(Predicate::<F>::False);

    //assert_eq!(Predicate::Not(true_pred).simplify(), Predicate::<F>::False);
    //assert_eq!(Predicate::Not(false_pred).simplify(), Predicate::<F>::True);
}