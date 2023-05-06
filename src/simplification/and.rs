use std::cmp::{min, max};

use num::Num;

use crate::predicate::{Predicate, Value};



pub fn simplify<T: Num + PartialOrd>(p1: Predicate<T>, p2: Predicate<T>) -> Predicate<T> {
    todo!()
}