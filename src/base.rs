use std::fmt;
use std::convert::{From, TryFrom};

extern crate rand;
use self::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// mod base { // goes into the base namespace by default, just from the filename.

// using DNA base pairs AGCT. T and C are always interchangeable in the last base of a codon.
// derive Debug for easy printing (otherwise would have to implement std::fmt::Display)
// can print like this: println!("{:?}", base);
#[derive(Debug, Copy)]
pub enum Base {
    C,
    T,
    A,
    G,
}

impl Clone for Base {
    fn clone(&self) -> Base { *self }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidBase(char),
}

impl TryFrom<char> for Base {
    type Error = self::ParseError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use self::Base::*;
        match c {
            'C' => Ok(C),
            'T' => Ok(T),
            'A' => Ok(A),
            'G' => Ok(G),
            x => Err(ParseError::InvalidBase(x)),
        }

    }
}

// could instead implement Into<char> for Base, but this is more idomatic
impl From<Base> for char {
    fn from(b: Base) -> char {
        use self::Base::*;
        match b {
            C => 'C',
            T => 'T',
            A => 'A',
            G => 'G',
        }        
    }
}

// impl Base {
//     pub fn to_char(&self) -> char {
//         use self::Base::*;
//         match *self {
//             C => 'C',
//             T => 'T',
//             A => 'A',
//             G => 'G',
//         }
//     }
// }

// good practice to implement this anyway
// println!("{}", base);
impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write!(f, "{}", self.into())
            write!(f, "{}", Base::from(*self))
    }
}

// impl Base {
//     pub fn from_char(c: char) -> Base {
//         use self::Base::*;
//         match c {
//             'C' => C,
//             'T' => T,
//             'A' => A,
//             'G' => G,
//             x => panic!("invalid character for base ({})", x),
//         }
//     }
// }

impl Distribution<Base> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Base {
        use self::Base::*;
        match rng.gen_range(0, 4) {
            0 => C,
            1 => T,
            2 => A,
            3 => G,
            _ => panic!("generated out-of-range number for random base"),
        }
    }
}

// should this be implemented as base.complement() ?
fn complement(base: Base) -> Base{
    use self::Base::*;
    match base {
        C => G,
        T => A,
        A => T,
        G => C,
    }
}

// } // mod base
