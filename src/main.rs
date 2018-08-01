// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]

mod base; // not clear -- are these being imported?
mod baseseq;
mod aminoacid;
mod protein;

use base::Base;
use baseseq::BaseSeq;
use protein::Protein;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

extern crate rand;
use rand::Rng;
use std::convert::TryFrom;

fn main() {
    // todo: pretty much all of this code can be eliminated, once we finish the translation function
    use Base::*;
    println!("Adenine: {}", A);
    println!("C(A): {}", base::complement(A));
    println!("ATG is start codon: {}", aminoacid::is_start_codon(&[A,T,G]));
    
    let mut rng = rand::thread_rng();
    let rndbase: Base = rng.gen();
    println!("random: {}", rndbase);

    // let b1 = BaseSeq::try_from("AGTCAGTCTA").expect("this should be valid");
    // let b2 = BaseSeq::try_from("TGCAGCTAGC").expect("this should be valid");
    // let bsum = b1.clone() + b2.clone();
    // println!("added sequence: {} + {} = {}", b1, b2, bsum);
    
    // let's implement something like this: (needs iterator for BaseSeq?)
    // need 3 kinds of iterators, w/ the default one provided by iter() to simply loop over 1 base at a time.
    // println!("complement:\t{}", bs.iter().map(|b| Base::complement(b)).collect());

    // let prots = protein::translate(bsum);
    // println!("translated:");
    // prots.iter().for_each(|p| println!("  {}", p) );

    // todo: better error description. should be able to catch the error and print it instead of catching w/ expect?
    // let inputs: Vec<String> = parse_args(env::args().collect()).expect("io error");
    let inputs: Vec<String> = parse_args(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    // let baseseqs: Vec<BaseSeq> = inputs.iter().map(|s: &String| BaseSeq::try_from(s.as_str())).collect::<Result<_,_>>().expect("parse error on input");
    // into_iter takes ownership
    let baseseqs: Vec<BaseSeq> = inputs.into_iter().map(BaseSeq::try_from).collect::<Result<_,_>>().unwrap_or_else(|err| {
        eprintln!("Problem parsing string as base sequence: {:?}", err);
        process::exit(1);
    });
    let prots: Vec<Protein> = baseseqs.into_iter().map(protein::translate).collect::<Vec<Vec<_>>>().concat();
    prots.iter().for_each(|p| println!("  {}", p) );
    
}

// takes std argv, removes the binary name, and reads the files if -f is included.
// returns a vector of strings to be parsed as base sequences.
fn parse_args(mut args: Vec<String>) -> Result<Vec<String>, std::io::Error> {
    args.remove(0); // the first element is the binary name
    // if the next element is the tag "-f", will open as filenames and attempt to read.
    let read_files = args.len() > 1 && &args[0] == "-f";
    if read_files {
        args.remove(0);
        let fins: Vec<File> =  args.into_iter().map(File::open).collect::<Result<_,_>>()?;
        let mut inputs: Vec<String> = Vec::new();
        for mut f in fins {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            inputs.extend(contents.split_whitespace().map(String::from));
        }
        Ok(inputs)
    } else {
        Ok(args)
    }
    
}
