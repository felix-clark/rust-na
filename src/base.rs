
use std::fmt;

extern crate rand;
use self::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// mod base { // goes into the base namespace by default, just from the filename.

// using DNA base pairs AGCT. T and C are always interchangeable in the last base of a codon.
// derive Debug for easy printing (otherwise would have to implement std::fmt::Display)
// can print like this: println!("{:?}", base);
#[derive(Debug)]
pub enum Base {
    C,
    T,
    A,
    G,
}
    
// good practice to implement this anyway
// println!("{}", base);
impl fmt::Display for Base {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Base::C => write!(b, "C"),
            Base::T => write!(b, "T"),
            Base::A => write!(b, "A"),
            Base::G => write!(b, "G"),
        }
    }
}

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
