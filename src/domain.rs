use num::Num;


/// Structure used to represent the "validity" domain of a Predicate with numerical arguments
/// This allows for easier code elsewhere in the library.
/// 
/// Basically, the Domain of a Predicate A describe the numerical space where each value verifies A.
pub struct Domain<T: Num> {
    lower: Option<T>,
    incl_lower: bool,

    greater: Option<T>,
    incl_greater: bool
}


impl<T: Num + PartialOrd + Clone> Domain<T> {

    // Helpful constructors
    pub fn point(x: T) -> Domain<T> {Domain { lower: Some(x.clone()), incl_lower: true, greater: Some(x), incl_greater: true }}
    pub fn _true() -> Domain<T> {Domain { lower: None, incl_lower: false, greater: None, incl_greater: false }}
    pub fn _false() -> Domain<T> {Domain { lower: Some(T::zero()), incl_lower: false, greater: Some(T::zero()), incl_greater: false }}

    pub fn new(lower: Option<T>, incl_lower: bool, greater: Option<T>, incl_greater: bool) -> Domain<T> {
        Domain { lower, incl_lower, greater, incl_greater }
    }

    pub fn intersection(d1: Domain<T>, d2: Domain<T>) -> Domain<T> {
        
        let (lower, incl_lower) = match (d1.lower, d2.lower) {
            (None, None) => (None, d1.incl_lower),
            (None, Some(x)) => (Some(x), d2.incl_lower),
            (Some(x), None) => (Some(x), d1.incl_lower),
            
            (Some(x1), Some(x2)) => {
                if x1 > x2 {(Some(x1), d1.incl_lower)}
                else if x1 < x2 {(Some(x2), d2.incl_lower)}
                else {(Some(x1), d1.incl_lower && d2.incl_lower)}
            }
        };

        let (greater, incl_greater) = match (d1.greater, d2.greater) {
            (None, None) => (None, d1.incl_greater),
            (None, Some(x)) => (Some(x), d2.incl_greater),
            (Some(x), None) => (Some(x), d1.incl_greater),
            
            (Some(x1), Some(x2)) => {
                if x1 < x2 {(Some(x1), d1.incl_greater)}
                else if x1 > x2 {(Some(x2), d2.incl_greater)}
                else {(Some(x1), d1.incl_greater && d2.incl_greater)}
            }
        };

        return Domain { lower, incl_lower, greater, incl_greater }

    }

}