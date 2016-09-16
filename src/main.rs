
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;

struct Decomposing<B> {
    buf: B,
    length: usize,
    index: usize,
}

trait Decomposable {
    fn substrings(self, length: usize) -> Decomposing<Self> where Self: Sized;
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
    
    let MIN_LEN = 3; 

    let mut white_counts: HashSet<&str> = HashSet::new();
    for k in &whitelist {
        // start with smallest substrings, up to full string
        for i in MIN_LEN..k.len() {
            for z in k.substrings(i) {
                white_counts.insert(z);
            }
        }
    }
	

    println!("[{}] white primitives (exhaustive)", white_counts.len());

    let mut black_counts: HashMap<&str, usize> = HashMap::new();
    for k in &blacklist {
        // start with smallest substrings, up to full string
        for i in MIN_LEN..k.len() {
            for z in k.substrings(i) {
                if white_counts.contains(z) {
                    continue;
                }
                let mut primitive_exists = false;
                for j in MIN_LEN..i {
                    if !primitive_exists {
                        for x in z.substrings(j) {
                            if !primitive_exists && black_counts.contains_key(x) {
                                primitive_exists = true;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                if !primitive_exists {
                    *black_counts.entry(z).or_insert(0) += 1;
                }
            }
        }
    }

    println!("[{}] black primitives (compact)", black_counts.len());

    let mut sorted_parts: Vec<(&&str, &usize)> = black_counts.iter().collect();
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
