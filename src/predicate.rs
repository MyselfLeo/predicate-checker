use std::fmt::{Display, Debug};

use num::{Num, ToPrimitive};

use crate::domain::Domain;


/// In a [Predicate], a value can either be a literal or an argument.
/// The latter are identified by a string, as there can be multiple arguments in a
/// given predicate.
#[derive(Debug, Clone, PartialEq)]
pub enum Value<T: Num + PartialOrd> {
    Arg(String),
    Literal(T)
}



/// A predicate is a boolean expression that can contain arguments with unknown values.
/// They can be simplified without evaluation.
/// 
/// It is possible to check if a given Predicate implies another Predicate, i.e. if for every
/// values where A is true, B is true for those values.
/// This is done without evaluating the Predicate.
#[derive(Debug, Clone, PartialEq)]
pub enum Predicate<T: Num + PartialOrd> {
    True,
    False,
    BoolArg(String),                        // identified by a string, not its value (as it is unknown)

    LowerThan(Value<T>, Value<T>),
    LowerEqual(Value<T>, Value<T>),
    GreaterThan(Value<T>, Value<T>),
    GreaterEqual(Value<T>, Value<T>),
    Equal(Value<T>, Value<T>),

    Not(Box<Predicate<T>>),
    And(Box<Predicate<T>>, Box<Predicate<T>>),
    Or(Box<Predicate<T>>, Box<Predicate<T>>)
}






impl<T: Num + PartialOrd + Clone + ToPrimitive + Display + Debug> Predicate<T> {

    /// Return the domain representing the values for which the predicate is true.
    /// If the predicate is based solely on arguments, the domain is unknown, so a full domain is returned.
    pub fn get_domain(&self, arg_name: String) -> Domain<T> {

        match self {
            Predicate::True => Domain::_true(),
            Predicate::False => Domain::_false(),
            Predicate::BoolArg(_) => Domain::_true(), // Unknown value => full domain

            Predicate::LowerThan(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(_), Value::Literal(x)) => Domain::new(None, false, Some(x.clone()), false),
                    (Value::Literal(x), Value::Arg(_)) => Domain::new(Some(x.clone()), false, None, false),
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 < x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::LowerEqual(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(_), Value::Literal(x)) => Domain::new(None, false, Some(x.clone()), true),
                    (Value::Literal(x), Value::Arg(_)) => Domain::new(Some(x.clone()), true, Some(x.clone()), false),
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 <= x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::GreaterThan(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(_), Value::Literal(x)) => Domain::new(Some(x.clone()), false, None, false),
                    (Value::Literal(x), Value::Arg(_)) => Domain::new(None, false, Some(x.clone()), false),
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 > x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::GreaterEqual(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(_), Value::Literal(x)) => Domain::new(Some(x.clone()), true, None, false),
                    (Value::Literal(x), Value::Arg(_)) => Domain::new(None, false, Some(x.clone()), true),
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 >= x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::Equal(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(_), Value::Literal(x)) => Domain::point(x.clone()),
                    (Value::Literal(x), Value::Arg(_)) =>  Domain::point(x.clone()),
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 == x2 {Domain::_true()} else {Domain::_false()},
                }
            },

            Predicate::Not(p) => Domain::complement(p.get_domain()),
            Predicate::And(p1, p2) => Domain::intersection(p1.get_domain(), p2.get_domain()),
            Predicate::Or(p1, p2) => Domain::union(p1.get_domain(), p2.get_domain()),
        }
    }


    /// Check that each value that satisfies the first Predicate also satisfies the second Predicate.
    /// In other terms, check if the Domain of the first Predicate is a subset of the Domain of the second Predicate.
    pub fn fits(&self, other: &Predicate<T>) -> bool {
        let d1 = self.get_domain().simplified();
        let d2 = other.get_domain().simplified();
        
        return Domain::union(d1, d2.clone()) == d2;
    }

}