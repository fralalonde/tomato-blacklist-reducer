
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::ops::Index;

struct Decomposing<B> {
    buf: B,
    length: usize,
    index: usize,
}

trait Decomposable {
    fn substrings(self, minLen: usize) -> Decomposing<Self> where Self: Sized;
}

impl<'a> Decomposable for &'a str {
    fn substrings(self, len: usize) -> Decomposing<Self>
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
        if self.index + self.length < self.buf.len() {
            let ref subs = self.buf[self.index..self.index + self.length];
            self.index += 1;
            Some(subs)
        } else {
            None
        }
    }
}

fn main() {
    let mut blacklist: Vec<String> = Vec::new();
    let mut whitelist: Vec<String> = Vec::new();

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
                Ok(line) => {
                    if b.starts_with("white") {
                        whitelist.push(format!("^{}$$", line))
                    } else {
                        blacklist.push(format!("^{}$$", line))
                    }
                }
                Err(_) => {}
            }
        }
    }

    println!("LINES white {} black {}", whitelist.len(), blacklist.len());

    let mut whitemains: HashMap<&str, usize> = HashMap::new();
    for k in &whitelist {
        // start with smallest substrings, up to full string
        for i in 3..k.len() {
            for z in k.substrings(i) {
                *whitemains.entry(z).or_insert(0) += 1;
            }
        }
    }

    let mut domains: HashMap<&str, usize> = HashMap::new();
    for k in &blacklist {
        // start with smallest substrings, up to full string
        for i in 3..k.len() {
            for z in k.substrings(i) {
                if !whitemains.contains_key(z) {
                    *domains.entry(z).or_insert(0) += 1;
                }
            }
        }
    }

    println!("UNIQUE SUBSTRINGS {}", domains.len());

    let mut sorted_parts: Vec<(&&str, &usize)> = domains.iter().collect();
    sorted_parts.sort_by(|a, b| b.1.cmp(a.1));

    for i in 0..20 {
        let (id, count) = *sorted_parts.index(i);
        println!("[{}] {} : {}", i, id, count);
    }

    for i in 0..200 {
        let (id, _) = *sorted_parts.index(i);
        println!("{}", id);
    }


}
