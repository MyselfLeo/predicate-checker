use num::Num;

use crate::predicate::Predicate;

pub fn simplify<T: Num + Clone + PartialOrd>(p: Predicate<T>) -> Predicate<T> {
    match p {
        Predicate::True => Predicate::False,
        Predicate::False => Predicate::True,
        o => o.clone()
    }
}