
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;

struct Decomposing<B> {
    buf: B,
    length: u32,
    index: u32,
}

trait Decomposable {
    fn substrings(self, minLen: u32) -> Decomposing<Self> where Self: Sized;
}

impl<'a> Decomposable for &'a str {
    fn substrings(self, len: u32) -> Decomposing<Self>
        where Self: Sized
    {
        Decomposing {
            buf: self,
            length: len,
            index: 0,
        }
    }
}

impl<'a> Iterator for Decomposing<&'a str> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.index += 1;
        if self.index < self.length {
            Some(&self.buf)
        } else {
            None
        }
    }
}

fn main() {
    let mut domain_list: Vec<String> = Vec::new();
    let mut domains: HashMap<&str, u32> = HashMap::new();

    let mut args = env::args();
    args.next(); // skip the program name

    for b in args {
        let path = Path::new(&b);
        let lines = File::open(path)
            .map(|f| BufReader::new(f))
            .map(|r| r.lines())
            .unwrap();
        for l in lines {
            match l {
                Ok(line) => domain_list.push(line),
                Err(_) => {}
            }
        }
    }

    println!("LINES {}", domain_list.len());
    
    for k in &domain_list {
        for z in k.substrings(3) {
            let count = domains.entry(z).or_insert(0);
            *count += 1;
        }
    }

    println!("LINES {}", domains.len());

}
