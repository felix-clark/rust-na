// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]
// #![feature(exact_chunks)]

mod base;
mod translate;
mod baseseq;
mod aminoacid;
mod protein;
mod fasta;
// mod basestream;

use baseseq::BaseSeq;
use base::Base;

use std::env;
use std::process;
use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;

extern crate itertools;

fn main() -> Result<(), std::io::Error> {
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
        let mut inputs: Vec<String> = Vec::new();
        for mut f in fins {
            let reader = fasta::Reader::new(f);
            let comb = itertools::concat(reader);
            // inputs.extend(reader.collect::<Vec<_>>()); // not efficient at all -- change this
            println!("{:?}", comb);
            let bs = comb.bytes().map(Base::try_from).collect::<Result<_,_>>()?;
            println!("{:?}", bs);
            baseseqs.push(bs);
        }
    }
    
    // let inputs: Vec<String> = parse_args(env::args().collect()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    
    // let baseseqs: Vec<BaseSeq> = inputs.into_iter().map(BaseSeq::try_from).collect::<Result<_,_>>().unwrap_or_else(|err| {
    //     eprintln!("Problem parsing string as base sequence: {:?}", err);
    //     process::exit(1);
    // });
    
    let prots = baseseqs.iter().flat_map(|seq| seq.translate());
    prots.filter(|p| p.len() >= 0).for_each( |p| println!("{}", p) );
    Ok(())
}

// // takes std argv, removes the binary name, and reads the files if -f is included.
// // returns a vector of strings to be parsed as base sequences.
// fn parse_args(args: Vec<String>) -> Result<Vec<String>, std::io::Error> {
//     let mut it_arg = args.into_iter().peekable(); // make peekable to check the first argument for the "-f" flag
//     it_arg.next(); // ignore the first element, which is the binary name
//     // if the next element is the tag "-f", will open as filenames and attempt to read.
//     let read_files = it_arg.peek() == Some(&String::from("-f"));
//     if read_files {
//         it_arg.next();
//         let fins: Vec<File> =  it_arg.map(File::open).collect::<Result<_,_>>()?;
//         let mut inputs: Vec<String> = Vec::new();
//         for mut f in fins {
//             let reader = fasta::Reader::new(f);
            
//             inputs.extend(reader.collect::<Vec<_>>()); // not efficient at all -- change this
//             // let mut contents = String::new();
//             // f.read_to_string(&mut contents)?;
//             // inputs.extend(contents.split_whitespace().map(String::from));
//         }
//         Ok(inputs)
//     } else {
//         Ok(it_arg.collect())
//     }    
// }
