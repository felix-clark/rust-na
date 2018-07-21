
use std::fmt;
    
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

fn complement(base: Base) -> Base{
    match base {
        Base::C => Base::G,
        Base::T => Base::A,
        Base::A => Base::T,
        Base::G => Base::C,
    }
}
    
// } // mod base
