
use std::io::{
    // BufRead,
    BufReader,
    // Lines,
    Result,
};
use std::fs::File;
use std::iter::Peekable;
// use std::marker::PhantomData;

extern crate itertools;
use self::itertools::concat;

#[derive(Debug)]
// pub struct Reader<I>
//     where I: Iterator<Item = String>
pub struct Reader
{
    // comment_buff: String, // we could save the comment of each sequence to peak at
    // buffer: String, // we might not even need this
    reader: BufReader<File>,
    // lines: Peekable<Lines<BufReader<File>>>,
    // lines: P where P: Peekable<I>,
    // lines: Lines<BufReader<File>>,
    // _marker: PhantomData<I>,
}

// impl<I> Reader<I>
//     where I: Iterator<Item = String>
impl Reader
{
    // pub fn new(f: File) -> self::Reader<I> {
    pub fn new(f: File) -> self::Reader {
        // let buffer = String::new(); // don't think we actually need our own buffer?
        // let reader = BufReader::new(f);
        // typical fasta has 80 bases in each line
        let reader = BufReader::with_capacity(80, f);
        // let mut lines = reader.lines().peekable();
        // println!("{:?}", lines);
        // let fstline = lines.next().expect("no first line").expect("failed to read first line");
        // println!("{:?}", fstline);
        // if lines.peek() == Some(b'>') {
        // println!("{:?}", lines.next());
        // println!("{:?}", lines);
        // }
        
        // self::Reader::<I> {
        self::Reader {
            // buffer,
            reader,
            // lines,
            // _marker: PhantomData,
        }
    }
    // this function is unstable. don't need it (yet?)
    // pub fn buffer(&self) -> &[u8] {
    //     self.reader.buffer()
    // }
}

struct ChunkReader<'a> {
    reader: &'a mut BufReader<File>
}

impl<'a> ChunkReader<'a> {
    pub fn new(reader: &'a mut BufReader<File>) -> ChunkReader {
        ChunkReader{reader}
    }
}

impl<'a> Iterator for ChunkReader<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// will yield one line at a time. it should return None when reaching a new sequence, then be reachable again.
// impl<I> Iterator for self::Reader<I>
//     where I: Iterator<Item = String>
impl Iterator for self::Reader
{
    type Item = ChunkReader;
    // type Item = I;
    // type Item = Lines<BufReader<File>>;
    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        let chunks = ChunkReader{reader: &mut self.reader};
        Some(chunks)
        // match self.lines.peek() {
        //     None => return None,
        //     _ => ()
        // }
        // let notcomment = |l: Option<Result<String>>| -> bool {
        // let notcomment = |l: Result<String>| -> bool {
        //     match l {
        //         // None => false,
        //         Err(e) => {
        //             eprintln!("error reading line: {}", e);
        //             false
        //         }
        //         Ok(x) => {
        //             ! x.starts_with(">")
        //         }
        //     }
        // };
        // let chunk = self.lines.by_ref().take_while(notcomment);
        // Some(chunk)
    }
}
