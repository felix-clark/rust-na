use base::*;

use std::convert::From;
use std::fmt;
use std::ops::Add;

// define a container for lists of bases.
pub struct BaseSeq {
    bs: Vec<Base>,
    // or should we use a VecDeque, or linked list?
}

impl BaseSeq {
    pub fn new() -> BaseSeq {
        BaseSeq {bs: Vec::new(),}
    }

    // // we'll actually probably want to move to implement From<string>
    // pub fn from_string(s: &str) -> BaseSeq {
    //     BaseSeq {
    //         bs: s.chars().map(|c| Base::from_char(c)).collect()
    //     }
    // }

    // moves all elements of other into self, leaving other empty
    pub fn append(&mut self, other: &mut BaseSeq) {
        // uses the built-in Vec::append(), which should hopefully pre-allocate for the length of other.bs
        self.bs.append(&mut other.bs);
    }

}

// the reference needs lifetime annotation
impl<'a> From<&'a str> for BaseSeq {
    // From<> shouldn't fail. we should use TryFrom, or Option<T> / Result<T, E>
    fn from(s: &'a str) -> BaseSeq {
        use std::convert::TryFrom;
        BaseSeq {
            // we'll want to return a Result<BaseSeq, Base::ParseError> instead of just calling unwrap() (which panics on an error)
            bs: s.chars().map(|c: char| Base::try_from(c).unwrap()).collect()
        }
    }
}

impl From<String> for BaseSeq {
    // again, should replace w/ TryFrom etc.
    // this is supposed to imply Into
    fn from(s: String) -> BaseSeq {
        use std::convert::TryFrom;
        // same comment here as above
        BaseSeq {bs: s.chars().map(|c| Base::try_from(c).unwrap()).collect()}
    }
}

impl fmt::Display for BaseSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let outstr: String = self.bs.iter().map(|b| (*b).into(): char).collect(); // need #![feature(type_ascription)] for this syntax
        // b has type iter(Base):
        let outstr: String = self.bs.iter().map(|b| char::from(*b)).collect();
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
