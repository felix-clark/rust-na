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
pub fn translate(seq: BaseSeq) -> Vec<Protein> {
    use base::Base;
    // trying to avoid explicit indexing, but switching between iterators and slices is a pain
    // maybe it is best to implement our own iterators, with a conversion between them
    let mut result = Vec::new();
    let mut win_iter = seq.bs.windows(3);
    let mut start = win_iter.skip_while(|c| !is_start_codon(c));
    println!("start codon is: {:?}", start); // we are at the right spot here, but how to get back to a slice?
    // let other = std::slice::ExactChunks::<Base>::from(start); // something like this would be nice, but it's not there
    // TODO: maybe we can manually implement From<ExactChunks<Base>> for Windows<Base> and vice versa
    // worst-case we just manually call next() twice... ugly tho
    
    println!("start.next() is: {:?}", start.next());
    println!("start.next() is: {:?}", start.next());
    println!("start.next() is: {:?}", start.next()); // not really what we want -- this only iterates up til the failure.
    // let next = start.collect::<BaseSeq>().exact_chunks(3);
    
    //let iss = is_start_codon(win_iter.next().unwrap());
    result
}

fn translate_start(seq: &BaseSeq) -> (Protein, &BaseSeq) {
    let result: Vec<AminoAcid> = Vec::new();
    // TODO: figure out how to keep reference to remaining base seqenence
    (Protein{aas: result}, seq)
}
