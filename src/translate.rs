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
    if bs.len() < 3 {
        return None
    }
    if head_is_start_codon(&bs) {
        Some(&bs)
    } else {
        next_start(&bs[1..])
    }
}

// might be nice to write this as an iterator over an amino acid
fn write_protein(bs: &[Base]) -> (Protein, &[Base]) {
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
