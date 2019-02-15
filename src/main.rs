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
// use std::mem::{size_of, size_of_val}; // only for print statements

extern crate itertools;
use itertools::Itertools;

// reads the first part of a string and check if it should be ignored
// the standard fasta comment indicator is '>'
fn is_comment_line(l: &String) -> bool {
    let mut peek = l.bytes();
    match peek.next() {
        Some(b'>') => true,
        _ => false,
    }
}

fn get_base_seqs(f: &File) -> io::Result<Vec<BaseSeq>> {
    // let reader = fasta::Reader::new(f);
    let reader = BufReader::new(f);
    let mut baseseqs = Vec::<BaseSeq>::new();
    let mut bs_buff = Vec::<Base>::new();
    for line in reader.lines() {
        let lr = line?;
        if is_comment_line(&lr) {
            if !bs_buff.is_empty() {
                baseseqs.push(
                    bs_buff.drain(..).map_into::<Base>().collect()
                );
            }
            // println!("comment line: {}", lr);
        } else {
            let bases = lr.bytes().map(Base::try_from).collect::<Result<Vec<_>,_>>()
                .unwrap_or_else(|err| {
                    eprintln!("problem parsing base: {:?}", err);
                    process::exit(1);
                });
            bs_buff.extend(bases);
        }
    }
    if !bs_buff.is_empty() {
        baseseqs.push(
            bs_buff.drain(..).map_into::<Base>().collect()
            );
    }
    Ok(baseseqs)
}

fn main() -> io::Result<()> {
    let mut it_arg = env::args().into_iter().peekable(); // make peekable to check the first argument for the "-f" flag
    it_arg.next(); // ignore the first element, which is the binary name
    // if the next element is the tag "-i", will attempt to read from stdin.
    let read_stdin = it_arg.peek() == Some(&String::from("-i"));

    let mut baseseqs: Vec<BaseSeq> = Vec::new(); //will change this later; should be streaming too
    if read_stdin {
        it_arg.next(); // skip the "-i"
        baseseqs = it_arg.map(BaseSeq::try_from).collect::<Result<_,_>>()
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing string as base sequence: {:?}", err);
                process::exit(1);
            });
    } else {
        let fins: Vec<File> =  it_arg.map(File::open).collect::<Result<_,_>>()?;    
        for mut f in fins {
            baseseqs.append(&mut get_base_seqs(&f)?);
        }
    }
    
    let prots = baseseqs.iter().flat_map(|seq| seq.translate());
    prots.filter(|p| p.len() >= 20).for_each( |p| println!("{}\n", p) );
    // prots.for_each( |p| println!("{}\n", p) );

    // this prints 2, so obviously 1 byte/base
    // we should be able to pack 4 bases per byte
    // println!("size of base: {}", size_of::<[Base;2]>());
    
    Ok(())
}
