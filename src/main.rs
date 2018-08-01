// -*- compile-command: "cargo build" -*-
// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]
// #![feature(exact_chunks)]

mod base;
mod translate;
mod baseseq;
mod aminoacid;
mod protein;

use baseseq::BaseSeq;
use protein::Protein;

use std::env;
use std::process;
use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;

fn main() {
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
    // let prots: Vec<Protein> = baseseqs.into_iter().map(protein::translate).collect::<Vec<Vec<_>>>().concat();
    // let prots: Vec<Protein> = baseseqs.for_each(|bs| bs.translate().iter().collect()).concat();
    let mut prots: Vec<Protein> = Vec::new();
    for bs in baseseqs {
        let newp: Vec<Protein> = bs.translate().collect();
        prots.extend(newp);
    }
    prots.iter().for_each(|p| println!("  {}", p) );
    
}

// takes std argv, removes the binary name, and reads the files if -f is included.
// returns a vector of strings to be parsed as base sequences.
fn parse_args(args: Vec<String>) -> Result<Vec<String>, std::io::Error> {
    let mut it_arg = args.into_iter().peekable(); // make peekable to check first
    // args.remove(0); // the first element is the binary name
    it_arg.next(); // ignore the first element
    // if the next element is the tag "-f", will open as filenames and attempt to read.
    // let read_files = args.len() > 1 && &args[0] == "-f";
    let read_files = it_arg.peek() == Some(&String::from("-f"));
    if read_files {
        it_arg.next();
        // args.remove(0);
        // let fins: Vec<File> =  args.into_iter().map(File::open).collect::<Result<_,_>>()?;
        let fins: Vec<File> =  it_arg.map(File::open).collect::<Result<_,_>>()?;
        let mut inputs: Vec<String> = Vec::new();
        for mut f in fins {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            inputs.extend(contents.split_whitespace().map(String::from));
        }
        Ok(inputs)
    } else {
        Ok(it_arg.collect())
    }
    
}
