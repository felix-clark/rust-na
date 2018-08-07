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
        while ! at_start_codon(&self.it) {
            let _remaining = self.it.next()?;
        }
        let prot: Protein = write_protein(&mut self.it);
        // we should consider filtering out small proteins: the smallest known is 20 amino acids long
        Some(prot)   
    }
}

fn at_start_codon(ib: &Iter<Base>) -> bool {
    ib.clone().take(3).eq(start_codon().iter())
}

// might be nice to write this with an iterator over amino acids
fn write_protein(bs: &mut Iter<Base>) -> Protein
{
    let is_not_stop_codon = |aa: &AminoAcid| {
        match aa {
            AminoAcid::STOP => false,
            _ => true,
        }
    };
    let prot: Protein = bs.tuples().map(|tup: (_,_,_)| amino_code(tup)).take_while(is_not_stop_codon).collect();
    prot
}
