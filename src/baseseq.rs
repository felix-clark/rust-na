use base::*;

use std::fmt;
use std::ops::Add;

// define a container for lists of bases.
pub struct BaseSeq {
    bs: Vec<Base>,
    // or should we use a VecDeque, or linked list?
}

impl BaseSeq {
    pub fn new(s: &str) -> BaseSeq {
    // pub fn from_string(s: &str) -> BaseSeq {
        BaseSeq {
            bs: s.chars().map(|c| Base::from_char(c)).collect()
        }
    }

    // moves all elements of other into self, leaving other empty
    pub fn append(&mut self, other: &mut BaseSeq) {
        // uses the built-in Vec::append(), which should hopefully pre-allocate for the length of other.bs
        self.bs.append(&mut other.bs);
    }

}

impl fmt::Display for BaseSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outstr: String = self.bs.iter().map(|b| (*b).to_char()).collect();
        write!(f, "{}", outstr)
    }
}

// creates a copy, so probably isn't optimal or even as good as append()
impl Add for BaseSeq {
    type Output = BaseSeq; // needed to define the result of adding two sequences
    fn add(self, other: BaseSeq) -> BaseSeq {
        let cap = self.bs.len() + other.bs.len();
        let mut vb: Vec<Base> = Vec::with_capacity(cap);
        vb = self.bs; // does this render self used up? do we need to clone again?
        vb.extend(other.bs.iter().cloned());
        BaseSeq {
            // makes a copy of each element in other; may not be optimal
            // bs: self.bs.extend(other.bs.iter().cloned())
            bs: vb,
        }
        
    }
}
