// in here should be the code to iterate over a base sequence and produce an iterator over proteins

use base::*;
use protein::*;
use aminoacid::*;

use std::iter::Iterator;
use std::slice::Iter;

extern crate itertools;
use self::itertools::Itertools; // using this trait gives us everything

// sort of does transcription and translation at once
#[derive(Debug)]
pub struct Translator<'a> {
    it: Iter<'a, Base>, // should these lifetimes be the same?
}

impl<'a> Translator<'a> {
    pub fn new(it: Iter<'a, Base>) -> Self {
        Translator{it}
    }
}

// // or as a trait?
// pub trait Translator<I,O>
// {
//     fn translate(&self) -> Box<Iterator<Item = O>>;
// }

impl<'a> Iterator for Translator<'a> {
    type Item = Protein;
    fn next(&mut self) -> Option<Protein> {
        // could use Option::and_then(f) to apply a function if an option exists,
        // self.it.by_ref().skip_while(|it: &Iter<Base>| ! at_start_codon(&it)); // function is on iterator, not base
        while ! at_start_codon(&self.it) {
            let remaining = self.it.next();
            if remaining == None {
                return None;
            }
        }
        let prot: Protein = write_protein(&mut self.it);
        // assert!(prot.len() != 0); // len() is not implemented for Protein
        Some(prot)   
    }
}

fn at_start_codon(ib: &Iter<Base>) -> bool {
    let topthr = ib.clone().take(3).cloned().collect::<Vec<_>>();
    is_start_codon(topthr.as_slice())
}


// might be nice to write this as an iterator over an amino acid
// fn write_protein<'a>(bs: &'a mut Iter<Base>) -> Iter<'a, AminoAcid>
fn write_protein(bs: &mut Iter<Base>) -> Protein
{
    let is_not_stop = |aa: &AminoAcid| {
        match aa {
            AminoAcid::STOP => false,
            _ => true,
        }
    };
    let prot: Protein = bs.tuples().map(|tup: (_,_,_)| amino_code(tup)).take_while(is_not_stop).collect();
    prot        
}
