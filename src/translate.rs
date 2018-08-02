// in here should be the code to iterate over a base sequence and produce an iterator over proteins

use base::*;
use protein::*;
use aminoacid::*;

// sort of does transcription and translation at once
#[derive(Debug)]
pub struct Translator<'a> {
    v: &'a [Base],
}

impl<'a> Translator<'a> {
    pub fn new(bs: &'a [Base]) -> Translator {
        Translator {
            v: &bs,
        }
    }
}

impl<'a> Iterator for Translator<'a> {
    type Item = Protein;

    fn next(&mut self) -> Option<Protein> {
        // could use Option::and_then(f) to apply a function if an option exists,
        //  but it might be safer to set self.v to an empty slice.
        match next_start(self.v) {
            None => {
                self.v = &[];
                None
            }
            Some(x) => {
                let (prot, rest) = write_protein(x);
                self.v = rest;
                Some(prot)
            }
        }
        
    }
}

fn next_start(bs: &[Base]) -> Option<&[Base]> {
    let x = bs.as_ref().windows(3);
    if bs.len() < 3 {
        return None
    }
    if head_is_start_codon(&bs) {
        Some(&bs)
    } else {
        next_start(&bs[1..])
    }
}

// // convenience function to use below
// fn not_stop(aa: AminoAcid) -> Option<AminoAcid> {
//     match aa {
//         AminoAcid::STOP => None,
//         x => Some(x),
//     }
// }

// private iterator struct to use below
// very similar to a Windows iter, but keeps track of remaining for retrieval later.
struct CodonIter<'a> {
    v: &'a [Base],
}
impl<'a> CodonIter<'a> {
    pub fn new(bs: &'a [Base]) -> CodonIter {
        CodonIter {
            v: &bs,
        }
    }
    // function to get the remaining sequence
    pub fn remaining(&self) -> &[Base] {
        // make sure we should be stopped
        assert!(self.v.len() < 3
                match amino_code(self.v[..3]) {
                    AminoAcid::STOP => true,
                    _ => false,
                });
        self.v
    }
}
impl<'a> Iterator for CodonIter<'a> {
    type Item = AminoAcid;
    fn next(&mut self) -> Option<Self::Item> {
        let step_size = 3;
        if self.v.len() < step_size {
            return None
        }
        let code = amino_code(&self.v[..step_size]);
        self.v = &self.v[step_size..];
        match code {
            AminoAcid::STOP => None,
            x => Some(x),
        }
    }
}


// might be nice to write this as an iterator over an amino acid
fn write_protein(bs: &[Base]) -> (Protein, &[Base]) {
    
    // let mut codit = CodonIter::new(bs);
    // // let result: Protein = codit.collect();
    // (codit.collect(), codit.remaining())

    
    // can likely use by_ref() to retain the iterator instead of letting it be discarded.
    
    // this might be a pretty ugly C-style function right now, just to get it working.
    let seqlen = bs.len();
    let step_size = 3;
    assert!(seqlen >= step_size);
    let mut steps: usize = 0;
    let mut result = Protein::new();
    let mut thisa: AminoAcid = amino_code(&bs[..step_size]);
    // awkward pattern matching to avoid deriving PartialEq for AminoAcid
    //  (tho this would not be a big problem)
    while
        steps + step_size <= seqlen &&
        match thisa {
            AminoAcid::STOP => false,
            _ => true
        }
    {
        result.push(thisa);
        steps += step_size;
        thisa = amino_code(&bs[(steps)..(steps+step_size)]);
    }    
    // we want a sort of "chunks" method, but we need to hang on to the rest as well.
    (result, &bs[steps..]) //  a placeholder
}
