use num::Num;



/// Part of a Domain. Represents a space between two values.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval<T: Num> {
    lower: Option<T>,
    incl_lower: bool,

    greater: Option<T>,
    incl_greater: bool
}



impl<T: Num + PartialOrd + Clone> Interval<T> {
    /// Constructor
    pub fn new(lower: Option<T>, incl_lower: bool, greater: Option<T>, incl_greater: bool) -> Interval<T> {
        Interval { lower, incl_lower, greater, incl_greater }
    }

    /// Return the intersection of two [Interval], or None if they don't intersect.
    pub fn intersection(d1: Interval<T>, d2: Interval<T>) -> Option<Interval<T>> {   

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

        if greater < lower {return None} // no intersection
        if lower == greater && !(incl_lower && incl_greater) {return None} // idem
        
        return Some(Interval::new(lower, incl_lower, greater, incl_greater))

    }


    /// Return the union of two [Interval], or None if they don't intersect/touch.
    pub fn union(d1: Interval<T>, d2: Interval<T>) -> Option<Interval<T>> {


        // d1 touches d2 on the left
        if d1.greater == d2.lower && (d1.incl_greater || d2.incl_lower) {
            return Some(Interval::new(d1.lower, d1.incl_lower, d2.greater, d2.incl_greater))
        }
        // d2 touches d1 on the left
        if d2.greater == d1.lower && (d2.incl_greater || d1.incl_lower) {
            return Some(Interval::new(d2.lower, d2.incl_lower, d1.greater, d1.incl_greater))
        }

        // Not touching and no intersection = no union
        if Interval::intersection(d1.clone(), d2.clone()).is_none() {return None}


        let (lower, incl_lower) = match (d1.lower, d2.lower) {
            (None, None) => (None, d1.incl_lower),
            (None, Some(x)) => (Some(x), d2.incl_lower),
            (Some(x), None) => (Some(x), d1.incl_lower),
            
            (Some(x1), Some(x2)) => {
                if x1 < x2 {(Some(x1), d1.incl_lower)}
                else if x1 > x2 {(Some(x2), d2.incl_lower)}
                else {(Some(x1), d1.incl_lower && d2.incl_lower)}
            }
        };

        let (greater, incl_greater) = match (d1.greater, d2.greater) {
            (None, None) => (None, d1.incl_greater),
            (None, Some(x)) => (Some(x), d2.incl_greater),
            (Some(x), None) => (Some(x), d1.incl_greater),
            
            (Some(x1), Some(x2)) => {
                if x1 > x2 {(Some(x1), d1.incl_greater)}
                else if x1 < x2 {(Some(x2), d2.incl_greater)}
                else {(Some(x1), d1.incl_greater && d2.incl_greater)}
            }
        };

        return Some(Interval::new(lower, incl_lower, greater, incl_greater))
    }
}






/// Structure used to represent the "validity" domain of a Predicate using a set of intervals.
/// 
/// Basically, the Domain of a Predicate A describe the numerical space where each value verifies A.
#[derive(Debug, Clone)]
pub struct Domain<T: Num> {
    parts: Vec<Interval<T>>
}



impl<T: Num + PartialOrd + Clone> Domain<T> {

    // Helpful constructors
    pub fn point(x: T) -> Domain<T> {
        Domain { parts: vec![Interval::new(Some(x.clone()), true, Some(x), true)] }
    }
    pub fn _true() -> Domain<T> {
        Domain { parts: vec![Interval::new(None, false, None, false)] }
    }
    pub fn _false() -> Domain<T> {
        Domain { parts: vec![] }
    }


    /// Create a new Domain with one Interval.
    pub fn new(lower: Option<T>, incl_lower: bool, greater: Option<T>, incl_greater: bool) -> Domain<T> {
        Domain { parts: vec![Interval::new(lower, incl_lower, greater, incl_greater)] }
    }


    /// Return true if the Domain is empty.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    
    /// Return the intersection of two [Domain].
    /// If they don't intersect, the result is a [Domain] with no [Interval].
    pub fn intersection(d1: Domain<T>, d2: Domain<T>) -> Domain<T> {
        let mut res = Domain { parts: vec![] };

        for i1 in d1.parts {
            for i2 in d2.parts.clone() {
                if let Some(i) = Interval::intersection(i1.clone(), i2.clone()) {
                    res.parts.push(i);
                }
            }
        }

        return res.simplified();
    }


    /// Return the union of two [Domain].
    pub fn union(d1: Domain<T>, d2: Domain<T>) -> Domain<T> {
        // TODO: optimize and prevent duplicates
        let mut res = Domain { parts: d1.parts.clone() };
        res.parts.extend(d2.parts.clone());
        return res.simplified();
    }



    /// Return the complement of a [Domain].
    pub fn complement(d: Domain<T>) -> Domain<T> {
        let mut res = Domain { parts: vec![] };

        let mut lower = None;
        let mut incl_lower = false;

        for i in d.parts {
            if let Some(greater) = i.greater {
                if let Some(lower) = lower {
                    res.parts.push(Interval::new(Some(lower), incl_lower, Some(greater.clone() ), !i.incl_greater));
                }
                lower = Some(greater);
                incl_lower = !i.incl_greater;
            }
        }

        if let Some(lower) = lower {
            res.parts.push(Interval::new(Some(lower), incl_lower, None, false));
        }

        return res.simplified();
    }



    /// Return a simplified [Domain] by merging adjacent [Interval]s.
    pub fn simplified(&self) -> Domain<T> {
        if self.parts.is_empty() {return self.clone()}

        let mut res: Domain<T> = self.clone();
        let mut i = 0;
        while i < res.parts.len() - 1 {
            let mut j = i + 1;
            while j < res.parts.len() {
                if let Some(union) = Interval::union(res.parts[i].clone(), res.parts[j].clone()) {
                    res.parts[i] = union;
                    res.parts.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        return res
    }
}



impl<T: Num + PartialOrd + Clone> PartialEq for Domain<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.parts.len() != other.parts.len() {return false}
        for i in 0..self.parts.len() {
            if self.parts[i] != other.parts[i] {return false}
        }
        return true
    }
}