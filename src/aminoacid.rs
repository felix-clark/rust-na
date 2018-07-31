// mod base;

use base::Base;
use std::convert::From;

#[derive(Debug,Clone)]
pub enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    STOP,
    // START, // same as Met; we don't actually need this?
}

impl<'a> From<&'a AminoAcid> for char {
    fn from(a: &AminoAcid) -> char {
        use self::AminoAcid::*;
        match a {
            Ala => 'A',
            Arg => 'R',
            Asn => 'N',
            Asp => 'D',
            Cys => 'C',
            Gln => 'Q',
            Glu => 'E',
            Gly => 'G',
            His => 'H',
            Ile => 'I',
            Leu => 'L',
            Lys => 'K',
            Met => 'M',
            Phe => 'F',
            Pro => 'P',
            Ser => 'S',
            Thr => 'T',
            Trp => 'W',
            Tyr => 'Y',
            Val => 'V',
            STOP => 'X', // not a real symbol. use None, if we roll w/ Option?
        }
    }
}

// these are just helper functions, make them pub once we use them
// this should actually probably be a function of a slice, not a tuple
pub fn is_start_codon(codon: (Base, Base, Base)) -> bool {
    use base::Base::*;
    match codon {
        // in fact, other codons can sometimes indicate a start, depending on nearby factors.
        // for this toy model we will keep it simple.
        (A,T,G) => true,
        _       => false
    }
}

// TODO: slice, not tuple (or both)
fn amino_code(codon: (Base, Base, Base)) -> AminoAcid {
    // use Base::G;
    use base::Base::*;
    use self::AminoAcid::*;
    match codon {
        (T,T,b) => match b {
            T | C => Phe, A | G => Leu
        },
        (C,T,_) => Leu,
        (A,T,b) => match b {
            G => Met, // this one also codes for START
            _ => Ile
        },
        (G,T,_) => Val,
        (T,C,_) => Ser,
        (C,C,_) => Pro,
        (A,C,_) => Thr,
        (G,C,_) => Ala,
        (T,A,b) => match b {
            T | C => Tyr, A | G => STOP
        },
        (C,A,b) => match b {
            T | C => His, A | G => Gln
        },
        (A,A,b) => match b {
            T | C => Asn, A | G => Lys
        },
        (G,A,b) => match b {
            T | C => Asp, A | G => Glu
        },
        (T,G,b) => match b {
            T | C => Cys, A => STOP, G => Trp
        },
        (C,G,_) => Arg,
        (A,G,b) => match b {
            T | C => Ser, A | G => Arg
        },
        (G,G,_) => Gly,
    }
}
