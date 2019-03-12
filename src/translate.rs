// -*- compile-command: "cargo build" -*-
// iteration over a base sequence and produce an iterator over proteins

use aminoacid::*;
use base::*;
use protein::*;

use std::iter::Iterator;
use std::slice::Iter;

extern crate itertools;
use self::itertools::Itertools; // using this trait gives us everything

// sort of does transcription and translation at once
#[derive(Debug)]
pub struct Translator<'a> {
    it: Iter<'a, Base>, // should these lifetimes be the same?
    // require an initial sequence before the start codon is looked for.
    // in prokaryotes this is the Shine Dalgarno sequence,
    // in eukaryotes it is the Kozak consensus sequence.
    // Some species have variations
    init_seq: Vec<Base>,
}

impl<'a> Translator<'a> {
    pub fn new(it: Iter<'a, Base>, init_seq: Vec<Base>) -> Self {
        Translator {
            it: it,
            init_seq: init_seq,
        }
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
        // first find an initiation sequence
        while !at_sequence(&self.it, &self.init_seq) {
            let _remaining = self.it.next()?;
        }
        // TODO: the S-D sequence indicates that the start codon should occur in about 6-8 spots.
        // there should be a counter that looks for the right
        while !at_start_codon(&self.it) {
            let _remaining = self.it.next()?;
        }
        let prot: Protein = write_protein(&mut self.it);
        // we should consider filtering out small proteins: the smallest known is 20 amino acids long
        Some(prot)
    }
}

fn at_sequence(ib: &Iter<Base>, seq: &[Base]) -> bool {
    ib.clone().take(seq.len()).eq(seq.iter())
}

fn at_start_codon(ib: &Iter<Base>) -> bool {
    ib.clone().take(3).eq(start_codon().iter())
}

fn is_not_stop_codon(aa: &AminoAcid) -> bool {
    match aa {
        AminoAcid::STOP => false,
        _ => true,
    }
}

// might be nice to write this with an iterator over amino acids
fn write_protein(bs: &mut Iter<Base>) -> Protein {
    let prot: Protein = bs
        .tuples()
        .map(amino_code)
        .take_while(is_not_stop_codon)
        .collect();
    prot
}
