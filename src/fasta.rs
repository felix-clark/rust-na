
use std::io::{BufRead,
              BufReader};
use std::fs::File;
// use std::iter::Iter;
use std::marker::PhantomData;

pub struct Reader<I>
    where I: Iterator<Item = String>
{
    // comment_buff: String, // we could save the comment of each sequence to peak at
    buffer: String, // we might not even need this
    reader: BufReader<File>,
    _marker: PhantomData<I>,
}

impl<I> Reader<I>
    where I:Iterator<Item = String>
{
    pub fn new(f: File) -> self::Reader<I> {
        let buffer = String::new(); // don't think we actually need our own buffer?
        // let reader = BufReader::new(f);
        // typical fasta has 80 bases in each line
        let reader = BufReader::with_capacity(80, f);
        self::Reader {
            buffer, reader,
            _marker: PhantomData,
        }
    }
    pub fn buffer(&self) -> Option<&String> {
        Some(&self.buffer)
    }
}

// will yield one line at a time. it should return None when reaching a new sequence, then be reachable again.
impl<I> Iterator for self::Reader<I>
    where I: Iterator<Item = String>
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        // self.reader.read_line(&mut self.buffer)?;
        // converts to an option, so this will ignore errors? probably not the best
        self.reader.read_line(&mut self.buffer).ok()?;
        // match self.reader.read_line(&mut self.buffer) {
        //     Err(err) => {
        //         eprint!("Error reading line: {}", err);
        //         return None
        //     }
        //     Ok(_) => ()
        // }
        let mut first = self.buffer.chars().peekable();
        match first.peek() {
            None => panic!("empty line? should we panic?"),
            // a '>' indicates a comment line, starting a new entry. could fill a comment buffer
            Some('>') => None,
            // can we get rid of this clone?
            _ => {
                println!("{}", self.buffer);
                Some(self.buffer.clone()) // valid line
            }
        }
    }
}
