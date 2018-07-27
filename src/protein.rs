use baseseq::BaseSeq;

use aminoacid::*;
use std::convert::From;
use std::fmt;

// a protein is a string of amino acids.
#[derive(Clone,Default)]
pub struct Protein {
    aas: Vec<AminoAcid>,
}

// define the short-format output (debug will print 3-letter indicaters)
impl fmt::Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aas.iter().map(char::from).collect::<String>())
    }
}

// will read through a base sequence and generate proteins using the canonical DNA codon code
pub fn sequence(bs: BaseSeq) -> Vec<Protein> {
    Vec::new() //Protein::default()
}
