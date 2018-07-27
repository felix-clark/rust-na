use base::*;

use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::Add;

// define a container for lists of bases.
#[derive(Clone,Default)]
pub struct BaseSeq {
    bs: Vec<Base>,
    // or should we use a VecDeque, or linked list?
}

impl BaseSeq {
    pub fn new() -> BaseSeq {
        BaseSeq {bs: Vec::new(),}
    }

    // moves all elements of other into self, leaving other empty
    pub fn append(&mut self, other: &mut BaseSeq) {
        // uses the built-in Vec::append(), which should hopefully pre-allocate for the length of other.bs
        self.bs.append(&mut other.bs);
    }

}

// the reference needs lifetime annotation
impl<'a> TryFrom<&'a str> for BaseSeq {
    type Error = ParseError; // from base
    fn try_from(s: &'a str) -> Result<Self, Self::Error> { // this lifetime specifier doesn't seem necessary
        // BaseSeq {bs: s.chars().map(|c| Base::try_from(c).expect("failed to interpret base sequence from string")).collect()}
        // Result implements FromIterator, so we can do this instead:
        // we can use the ? operator but collect() needs this awkward "turbofish" syntax:
        let tryread = s.chars().map(Base::try_from).collect::<Result<_,_>>()?;
        Ok(BaseSeq{bs: tryread})
    }
}

impl TryFrom<String> for BaseSeq {
    // this implies TryInto
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let tryread = s.chars().map(Base::try_from).collect::<Result<_,_>>()?;
        Ok(BaseSeq{bs: tryread})
    }
}

impl From<BaseSeq> for String {
    fn from(bs: BaseSeq) -> String {
        bs.bs.iter().map(|b| char::from(*b)).collect()
    }
}

// we need the reference version to print a string w/out
impl<'a> From<&'a BaseSeq> for String {
    fn from(seq: &BaseSeq) -> String {
        seq.bs.iter().map(|b| char::from(*b)).collect()
    }
}

impl fmt::Display for BaseSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // b has type iter(Base):
        // let outstr: String = self.bs.iter().map(|b| (*b).into(): char).collect(); // need #![feature(type_ascription)] for this syntax
        // let outstr: String = self.bs.iter().map(|b| char::from(*b)).collect();
        write!(f, "{}", String::from(self))
    }
}

// creates a copy, so probably isn't optimal or even as good as append()
impl Add for BaseSeq {
    type Output = BaseSeq; // needed to define the result of adding two sequences
    fn add(self, other: BaseSeq) -> BaseSeq {
        let cap = self.bs.len() + other.bs.len();
        let mut vb: Vec<Base> = Vec::with_capacity(cap);
        vb = self.bs; // does this render self used up? do we need to clone again?
        // makes a copy of each element in other; may not be optimal
        vb.extend(other.bs.iter().cloned());
        BaseSeq {
            bs: vb,
        }
        
    }
}
