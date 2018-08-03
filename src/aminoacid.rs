// mod base;

use base::Base;
use std::convert::From;

/// represents the standard proteinogenic amino acids.
/// the exotic ones (like selenocysteine) are not included
/// because they are not simply mapped from a single codon
/// (typically one of the STOP codons, but conditionally).
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
    // START, // redundant with Met; all proteins will being with M in this simple model
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
            STOP => '*',
            // a gap of indeterminate length is symbolized by '-'; not clear we need that here
        }
    }
}

// these are just helper functions, make them pub once we use them
// this should actually probably be a function of a slice, not a tuple
// pub fn is_start_codon(codon: (Base, Base, Base)) -> bool {
pub fn is_start_codon(codon: &[Base]) -> bool {
    use base::Base::*;
    match codon {
        [A,T,G] => true,
        _       => false,
    }
}

pub fn amino_code(codon: (&Base, &Base, &Base)) -> AminoAcid {
    use base::Base::*;
    use self::AminoAcid::*;
    match codon { // somewhat awkward; may want to match to slice patter for everything below
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
