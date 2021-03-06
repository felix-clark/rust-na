// add this to lib in case we export later, but put it in main so we can use it here.
// #![feature(exact_chunks)]

mod aminoacid;
mod base;
mod baseseq;
mod protein;
mod translate;
// mod fasta;
// mod basestream;

use base::Base;
use baseseq::BaseSeq;

use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io;
use std::io::{
    // Read,
    BufRead,
    BufReader,
};
use std::process;
// use std::mem::{size_of, size_of_val}; // only for print statements

extern crate itertools;
use itertools::Itertools;

// reads the first part of a string and check if it should be ignored
// the standard fasta comment indicator is '>'
fn is_comment_line(l: &str) -> bool {
    let mut peek = l.bytes();
    match peek.next() {
        Some(b'>') => true,
        _ => false,
    }
}

fn get_base_seqs(f: File) -> io::Result<Vec<BaseSeq>> {
    // let reader = fasta::Reader::new(f);
    let reader = BufReader::new(f);
    let mut baseseqs = Vec::<BaseSeq>::new();
    let mut bs_buff = Vec::<Base>::new();
    for line in reader.lines() {
        let lr = line?;
        if is_comment_line(&lr) {
            if !bs_buff.is_empty() {
                baseseqs.push(bs_buff.drain(..).map_into::<Base>().collect());
            }
        // println!("comment line: {}", lr);
        } else {
            let bases = lr
                .bytes()
                .map(Base::try_from)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|err| {
                    eprintln!("problem parsing base: {:?}", err);
                    process::exit(1);
                });
            bs_buff.extend(bases);
        }
    }
    if !bs_buff.is_empty() {
        baseseqs.push(bs_buff.drain(..).map_into::<Base>().collect());
    }
    Ok(baseseqs)
}

fn main() -> io::Result<()> {
    let mut it_arg = env::args().peekable(); // make peekable to check the first argument for the "-f" flag
    it_arg.next(); // ignore the first element, which is the binary name
                   // if the next element is the tag "-i", will attempt to read from stdin.
    let read_stdin = it_arg.peek() == Some(&String::from("-i"));

    //will change this later; should be a streaming iterator too
    let baseseqs: Vec<BaseSeq> = if read_stdin {
        it_arg.next(); // skip the "-i"
        it_arg
            .map(BaseSeq::try_from)
            .collect::<Result<_, _>>()
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing string as base sequence: {:?}", err);
                process::exit(1);
            })
    } else {
        let fins: Vec<File> = it_arg.map(File::open).collect::<Result<_, _>>()?;
        fins.into_iter()
            // this ends up with a vec of vecs, which may not be optimal
            // but we have to collect a result at each stage
            .map(get_base_seqs)
            .collect::<io::Result<Vec<_>>>()?
            .concat()
    };

    // Shine-Dalgarno variation for E. coli
    use Base::*;
    let init_seq = vec![A, G, G, A, G, G, T];
    // let init_seq = vec![];
    let prots = baseseqs
        .iter()
        .flat_map(|seq| seq.translate(init_seq.clone())); // shouldn't need to clone here
    prots
        .filter(|p| p.len() >= 20)
        .for_each(|p| println!("{}\n", p));
    // prots.for_each( |p| println!("{}\n", p) );

    // this prints 2, so obviously 1 byte/base
    // we should be able to pack 4 bases per byte
    // println!("size of base: {}", size_of::<[Base;2]>());

    Ok(())
}
