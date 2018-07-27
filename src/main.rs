// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]

mod base; // not clear -- are these being imported?
mod baseseq;
mod aminoacid;

use base::Base;
use baseseq::BaseSeq;

extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello i guess");
    println!("{}", Base::A);
    let mut rng = rand::thread_rng();
    let rndbase: Base = rng.gen();
    println!("random: {}", rndbase);

    // let bs = BaseSeq::from_string("ATCGCAT");
    let bs = BaseSeq::from("ATCGCAT");
    println!("sequence:\t{}", bs);
    // let's implement something like this:
    // println!("complement:\t{}", bs.iter().map(|b| Base::complement(b)).collect());
}
