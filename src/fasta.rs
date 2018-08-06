
use std::io::{BufRead,
              BufReader};
use std::fs::File;

pub struct Reader {
    // comment_buff: String, // we could save the comment of each sequence to peak at
    buffer: String,
    reader: BufReader<File>,
}

impl Reader {
    pub fn new(f: File) -> self::Reader {
        let buffer = String::new();
        let reader = BufReader::new(f);
        self::Reader {
            buffer, reader
        }
    }
    pub fn buffer(&self) -> Option<&String> {
        Some(&self.buffer)
    }
}

// will yield one line at a time. it should return None when reaching a new sequence, then be reachable again.
// impl self::Reader {
impl Iterator for self::Reader {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Err(err) => {
                eprint!("Error reading line: {}", err);
                return None
            }
            Ok(_) => ()
        }
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
