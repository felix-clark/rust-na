// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]
// #![feature(exact_chunks)]

mod base;
mod translate;
mod baseseq;
mod aminoacid;
mod protein;
// mod fasta;
// mod basestream;

use baseseq::BaseSeq;
use base::Base;

use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::{
    // Read,
    BufRead,
    BufReader};
use std::convert::TryFrom;

// extern crate itertools;
// use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut it_arg = env::args().into_iter().peekable(); // make peekable to check the first argument for the "-f" flag
    it_arg.next(); // ignore the first element, which is the binary name
    // if the next element is the tag "-i", will attempt to read from stdin.
    let read_stdin = it_arg.peek() == Some(&String::from("-i"));

    let mut baseseqs: Vec<BaseSeq> = Vec::new(); //will change this later; should be streaming too
    if read_stdin {
        it_arg.next(); // skip the "-i"
        baseseqs = it_arg.map(BaseSeq::try_from).collect::<Result<_,_>>().unwrap_or_else(|err| {
            eprintln!("Problem parsing string as base sequence: {:?}", err);
            process::exit(1);
        });
    } else {
        let fins: Vec<File> =  it_arg.map(File::open).collect::<Result<_,_>>()?;
        for mut f in fins {
            // let reader = fasta::Reader::new(f);
            let reader = BufReader::new(f);
            let comment_line = |l: &String| -> bool {
                let mut peek = l.bytes();
                match peek.next() {
                    Some(b'>') => true,
                    _ => false,
                }
            };

            // let mut bs_buff = BaseSeq::new();
            let mut bs_buff: Vec<Base> = Vec::new();
            for line in reader.lines() {
                let lr = line?;
                if comment_line(&lr) {
                    if !bs_buff.is_empty() {
                        // clone is not efficient, but let's get it working first
                        baseseqs.push(BaseSeq::from(bs_buff.clone()));
                        bs_buff.clear();
                    }
                    println!("comment line: {}", lr);
                    // baseseqs.push(BaseSeq::new());
                } else {
                    let bases = lr.bytes().map(Base::try_from).collect::<Result<Vec<_>,_>>()
                        .unwrap_or_else(|err| {
                            eprintln!("problem parsing base: {:?}", err);
                            process::exit(1);
                        });
                    bs_buff.extend(bases);
                    // println!("{:?}", bs_buff);
                }
            }
            if !bs_buff.is_empty() {
                // clone is not efficient, but let's get it working first
                baseseqs.push(BaseSeq::from(bs_buff.clone()));
                bs_buff.clear();
            }

        }
    }
    
    let prots = baseseqs.iter().flat_map(|seq| seq.translate());
    prots.filter(|p| p.len() >= 20).for_each( |p| println!("{}", p) );
    // prots.for_each( |p| println!("{}", p) );
    Ok(())
}
