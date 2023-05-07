use std::{fmt::{Display, Debug}, collections::HashSet};

use num::{Num, ToPrimitive};

use crate::domain::Domain;
use crate::parser::parse_predicate;


/// In a [Predicate], a value can either be a literal or an argument.
/// The latter are identified by a string, as there can be multiple arguments in a
/// given predicate.
#[derive(Debug, Clone, PartialEq)]
pub enum Value<T: Num + PartialOrd> {
    Arg(String),
    Literal(T)
}



/// A predicate is a boolean expression that can contain arguments with unknown values.
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




impl Predicate<f64> {
    /// Return a predicate from an infix predicate string.
    /// 
    /// # Example
    /// ```
    /// use predicatechecker::Predicate;
    /// 
    /// let p = Predicate::from("(x > 5) && (x < 10)").unwrap();
    /// ```
    pub fn from(txt: &str) -> Result<Predicate<f64>, String> {
        parse_predicate(txt)
    }
}



impl<T: Num + PartialOrd + Clone + ToPrimitive + Display + Debug> Predicate<T> {
    /// Return the domain representing the values of the given argument where the Predicate is true.
    /// Thus, the "validity domain" of a predicate is made of one domain for each of its arguments.
    /// A Predicate with no arguments could be simplified to a single boolean value, so this function would not be useful.
    pub fn get_domain(&self, arg_name: &str) -> Domain<T> {

        match self {
            Predicate::True => Domain::_true(),
            Predicate::False => Domain::_false(),

            // if _ is arg_name: unknown value so the validity domain could be anything
            // if _ is not arg_name: the value of arg_name is irrelevant so the domain of validity is any values
            Predicate::BoolArg(_) => Domain::_true(),

            Predicate::LowerThan(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(a1), Value::Arg(a2)) => {
                        if a1 == a2 {Domain::_false()} else {Domain::_true()} // the same argument cannot be lower than itself, so the Predicate is always false
                    },

                    (Value::Arg(a), Value::Literal(x)) => {
                        if a == arg_name {Domain::new(None, false, Some(x.clone()), false)}
                        else {Domain::_true()}
                    },

                    (Value::Literal(x), Value::Arg(a)) => {
                        if a == arg_name {Domain::new(Some(x.clone()), false, None, false)}
                        else {Domain::_true()}
                    },

                    (Value::Literal(x1), Value::Literal(x2)) => if x1 < x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::LowerEqual(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),

                    (Value::Arg(a), Value::Literal(x)) => {
                        if a == arg_name {Domain::new(None, false, Some(x.clone()), true)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x), Value::Arg(a)) => {
                        if a == arg_name {Domain::new(Some(x.clone()), true, Some(x.clone()), false)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 <= x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::GreaterThan(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(a1), Value::Arg(a2)) => {
                        if a1 == a2 {Domain::_false()} else {Domain::_true()} // the same argument cannot be greater than itself, so the Predicate is always false
                    },

                    (Value::Arg(a), Value::Literal(x)) => {
                        if a == arg_name {Domain::new(Some(x.clone()), false, None, false)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x), Value::Arg(a)) => {
                        if a == arg_name {Domain::new(None, false, Some(x.clone()), false)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 > x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::GreaterEqual(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(a), Value::Literal(x)) => {
                        if a == arg_name {Domain::new(Some(x.clone()), true, None, false)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x), Value::Arg(a)) => {
                        if a == arg_name {Domain::new(None, false, Some(x.clone()), true)}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 >= x2 {Domain::_true()} else {Domain::_false()},
                }
            },


            Predicate::Equal(v1, v2) => {
                match (v1, v2) {
                    (Value::Arg(_), Value::Arg(_)) => Domain::_true(),
                    (Value::Arg(a), Value::Literal(x)) => {
                        if a == arg_name {Domain::point(x.clone())}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x), Value::Arg(a)) => {
                        if a == arg_name {Domain::point(x.clone())}
                        else {Domain::_true()}
                    },
                    (Value::Literal(x1), Value::Literal(x2)) => if x1 == x2 {Domain::_true()} else {Domain::_false()},
                }
            },

            Predicate::Not(p) => Domain::complement(p.get_domain(arg_name)),
            Predicate::And(p1, p2) => Domain::intersection(p1.get_domain(arg_name), p2.get_domain(arg_name)),
            Predicate::Or(p1, p2) => Domain::union(p1.get_domain(arg_name), p2.get_domain(arg_name)),
        }
    }





    /// Return the set of arguments used by the predicate
    pub fn get_arguments(&self) -> HashSet<String> {
        match self {
            Predicate::True => HashSet::new(),
            Predicate::False => HashSet::new(),
            Predicate::BoolArg(a) => {
                let mut set = HashSet::new();
                set.insert(a.clone());
                set
            },
            Predicate::LowerThan(v1, v2) => {
                let mut set = HashSet::new();
                match v1 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                match v2 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                set
            },
            Predicate::LowerEqual(v1, v2) => {
                let mut set = HashSet::new();
                match v1 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                match v2 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                set
            },
            Predicate::GreaterThan(v1, v2) => {
                let mut set = HashSet::new();
                match v1 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                match v2 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                set
            },
            Predicate::GreaterEqual(v1, v2) => {
                let mut set = HashSet::new();
                match v1 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                match v2 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                set
            },
            Predicate::Equal(v1, v2) => {
                let mut set = HashSet::new();
                match v1 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                match v2 {
                    Value::Arg(a) => set.insert(a.clone()),
                    Value::Literal(_) => false,
                };
                set
            },
            Predicate::Not(p) => p.get_arguments(),
            Predicate::And(p1, p2) => {
                let mut set = p1.get_arguments();
                for a in p2.get_arguments() {set.insert(a);}
                set
            },
            Predicate::Or(p1, p2) => {
                let mut set = p1.get_arguments();
                for a in p2.get_arguments() {set.insert(a);}
                set
            },
        }
    }






    /// A Predicate A "fits" a Predicate B if:
    /// - B uses only arguments used by A
    /// - For every argument used by both A and B, the validity domain of A is a subset of the validity domain of B
    pub fn fits(&self, other: &Predicate<T>) -> bool {
        // A is self, B is other

        let self_args = self.get_arguments();
        let other_args = other.get_arguments();

        // Check that B uses only arguments used by A
        for a in other_args.iter() {
            if !self_args.contains(a) {return false;}
        }

        // Check validity domains for arguments used both by A and B
        for a in other_args.iter() {
            let d1 = self.get_domain(a);
            let d2 = other.get_domain(a);

            if Domain::union(d1, d2.clone()) != d2 {return false;}
        }

        true
    }
}