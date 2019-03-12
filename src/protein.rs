use aminoacid::*;
use std::convert::From;
use std::fmt;
use std::iter::FromIterator;

// a protein is a string of amino acids.
#[derive(Clone, Debug)]
pub struct Protein {
    aas: Vec<AminoAcid>,
}

impl Protein {
    pub fn new() -> Protein {
        Protein { aas: Vec::new() }
    }
    pub fn push(&mut self, a: AminoAcid) {
        self.aas.push(a);
    }
    pub fn len(&self) -> usize {
        self.aas.len()
    }
}

// clippy recommends implementing Default
impl Default for Protein {
    fn default() -> Self {
        Self::new()
    }
}

// define the short-format output (debug will print 3-letter indicaters)
impl fmt::Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aas.iter().map(char::from).collect::<String>())
    }
}

impl FromIterator<AminoAcid> for Protein {
    fn from_iter<I: IntoIterator<Item = AminoAcid>>(iter: I) -> Self {
        // let aas: Vec<AminoAcid> = iter.collect();
        // Protein{aas}
        // this is a bit verbose and unweidly...
        let mut aas = Protein::new();
        for i in iter {
            aas.push(i);
        }
        aas
    }
}
