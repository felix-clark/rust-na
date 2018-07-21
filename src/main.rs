// -*- compile-command: "cargo build" -*-
mod base;// not clear -- are these being imported?
mod aminoacid;

fn main() {
    println!("Hello i guess");
    println!("{}", base::Base::A);
}
