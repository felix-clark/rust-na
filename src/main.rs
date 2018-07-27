// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]

mod base; // not clear -- are these being imported?
mod baseseq;
mod aminoacid;
mod protein;

use base::Base;
use baseseq::BaseSeq;

extern crate rand;
use rand::Rng;
// use std::convert::{From, TryFrom};
use std::convert::TryFrom;

fn main() {
    println!("Adenine: {}", Base::A);
    let mut rng = rand::thread_rng();
    let rndbase: Base = rng.gen();
    println!("random: {}", rndbase);

    let bs: BaseSeq = BaseSeq::try_from("ATCGCAT").expect("this should be a valid sequence");
    println!("sequence:\t{}", bs);

    let b1 = BaseSeq::try_from("AGTCAGTCTA").expect("this should be valid");
    let b2 = BaseSeq::try_from("TGCAGCTAGC").expect("this should be valid");
    let bsum = b1.clone() + b2.clone();
    println!("added sequence: {} + {} = {}", b1, b2, bsum);
    
    // let's implement something like this: (needs iterator for BaseSeq?)
    // println!("complement:\t{}", bs.iter().map(|b| Base::complement(b)).collect());

    let prots = protein::sequence(bsum);
    println!("sequenced:");
    prots.iter().for_each(|p| println!("  {}", p) );
    
}
