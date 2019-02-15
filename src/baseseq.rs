use base::*;
// use aminoacid::AminoAcid;
use translate::*;

use std::convert::{From, TryFrom};
use std::fmt;
// use std::ops::Add;
use std::slice::{Iter};
use std::iter::{Iterator,
                FromIterator,
};

// define a container for lists of bases.
// derive Debug so we can use assertions in test
// this struct should be used for storing sequences; BaseStream should be used for reading from a file to process.
#[derive(Clone,Default,Debug,PartialEq)]
pub struct BaseSeq {
    bs: Vec<Base>,
}

impl BaseSeq {
    // pub fn new(it: I) -> BaseSeq
    // where I: Iterator<Item=String>
    pub fn new() -> BaseSeq
    {
        BaseSeq {bs: Vec::new(),}
    }

    // allows simple iteration base-by-base
    pub fn iter(&self) -> Iter<Base> {
        self.bs.iter()
    }

    pub fn push(&mut self, b: Base) {
        self.bs.push(b);
    }
    
    // moves all elements of other into self, leaving other empty
    pub fn append(&mut self, other: &mut BaseSeq) {
        // uses the built-in Vec::append(), which should hopefully pre-allocate for the length of other.bs
        self.bs.append(&mut other.bs);
    }

    // returns an iterator that will read the sequence and return the proteins
    // modeling the job of mRNA
    pub fn translate(&self, init_seq: Vec<Base>) -> Translator {
        Translator::new(self.bs.iter(), init_seq)
    }

    pub fn complement(&self) -> BaseSeq {
        self.iter().map(|b| complement(*b)).collect()
    }
    pub fn strength(&self) -> i32 {
        self.iter().fold(0, |x,b| x + strength(*b))
    }
    pub fn len(&self) -> usize {
        self.bs.len()
    }
}

// TODO: implement iterator adaptor?
// impl Translator<Base, AminoAcid> for BaseSeq {
//     // Iterator is not a type, so we need
//     fn translate(&self) -> Box<Iterator<Item = AminoAcid>> {
//         empty::<AminoAcid>()
//     }
// }

impl From<Vec<Base>> for BaseSeq {
    fn from(bs: Vec<Base>) -> BaseSeq {
        BaseSeq{bs}
    }
}

// the reference needs lifetime annotation
impl<'a> TryFrom<&'a str> for BaseSeq {
    type Error = ParseError; // from base
    fn try_from(s: &str) -> Result<Self, Self::Error> { // this lifetime specifier doesn't seem necessary
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
        write!(f, "{}", String::from(self))
    }
}

impl FromIterator<Base> for BaseSeq {
    // fn from_iter<I: IntoIterator<Item=Base>>(iter: I) -> Self {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=Base>,
    {
        let mut bs = BaseSeq::new();
        for i in iter {
            bs.push(i);
        }
        bs
    }
}

// // creates a copy, so probably isn't optimal or even as good as append()
// // in fact we don't even want this. we can add Vec<Base> if we really want.
// impl Add for BaseSeq {
//     type Output = BaseSeq; // needed to define the result of adding two sequences
//     fn add(self, other: BaseSeq) -> BaseSeq {
//         let mut vb: Vec<Base> = self.bs;
//         vb.reserve(other.bs.len());
//         // makes a copy of each element in other; may not be optimal
//         vb.extend(other.bs.iter().cloned());
//         BaseSeq {
//             bs: vb,
//         }        
//     }
// }

impl Extend<Base> for BaseSeq {
    fn extend<T: IntoIterator<Item=Base>>(&mut self, iter: T) {
        for elem in iter {
            self.bs.push(elem);
        }
    }
}
