// for reading bases from strings in sequence, without storing
// should implement lazy conversion and reading

use base::*;
use fasta::Reader;
use std::str::Chars;

pub struct BaseStream<'s> {
    read: Reader<Iterator<Item = String>>,
    // strbuff: &'s String,
    buffer: Vec<Base>,
    strit: Option<Chars<'s>>,
}

// impl BaseStream {
//     pub fn new(str_it: &Iterator<Item = String>) -> BaseStream {

//     }
// }

impl<'s, I> From<Reader<I>> for BaseStream<'s>
    where I: Iterator<Item = String>
{
    fn from(read: Reader<I>) -> BaseStream<'s> {
        BaseStream{read,
                   buffer: Vec::with_capacity(80),
                   strit: None}
    }
}

impl<'s> Iterator for BaseStream<'s> {
    type Item = Base;
    fn next(&mut self) -> Option<Base> {
        None // placeholder
    }
}
