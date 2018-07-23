use base::*;

use std::fmt;

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
}

impl fmt::Display for BaseSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outstr: String = self.bs.iter().map(|b| (*b).to_char()).collect();
        write!(f, "{}", outstr)
    }
}
