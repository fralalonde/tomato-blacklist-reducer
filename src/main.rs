
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

fn maybe_ip_address(addr: &str) -> bool {
    let ip_chars = "0123456789.";
    for c in addr.chars() {
        if !ip_chars.contains(c) {
            return false;
        }
    }
    true
}

fn build_white_counts(whitelist: &Vec<String>, min_length: usize) -> HashSet<&str> {
    let mut white_counts: HashSet<&str> = HashSet::new();
    for k in whitelist {
        // start with smallest substrings, up to full string
        for i in min_length..k.len() {
            'next_substring: for z in k.substrings(i) {
                for j in min_length..i {
                    for x in z.substrings(j) {
                        if white_counts.contains(x) {
                            continue 'next_substring;
                        }
                    }
                }
                white_counts.insert(z);
            }
        }
    }
    white_counts
}

fn load_lists() -> (Vec<String>, Vec<String>) {
    let mut blacklist: Vec<String> = Vec::new();
    let mut whitelist: Vec<String> = Vec::new();

    let mut args = env::args();
    args.next(); // skip the program name

    fn load_lines(b: String, list: &mut Vec<String>) {
        let path = Path::new(&b);
        let lines = File::open(path)
            .map(|f| BufReader::new(f))
            .map(|r| r.lines())
            .unwrap();
        for l in lines {
            match l {
                Ok(line) => {
                    if !maybe_ip_address(&line) {
                        list.push(format!("^{}$$", line))
                    }
                }
                Err(_) => {}
            }
        }
    }

    for b in args {
        if b.starts_with("white") {
            load_lines(b, &mut whitelist);
        } else {
            load_lines(b, &mut blacklist);
        };
    }
    (whitelist, blacklist)
}

fn main() {

    let (whitelist, blacklist) = load_lists();

    println!("LINES white {} black {}", whitelist.len(), blacklist.len());

    let min_length = 6;

    let white_counts = build_white_counts(&whitelist, min_length);

    println!("[{}] white primitives", white_counts.len());

    let mut black_counts: HashMap<&str, usize> = HashMap::new();
    for k in &blacklist {
        // start with smallest substrings, up to full string
        for i in min_length..k.len() {
            'outer: for z in k.substrings(i) {
                if white_counts.contains(z) {
                    continue 'outer;
                }
                for j in min_length..i {
                    for x in z.substrings(j) {
                        if white_counts.contains(x) || black_counts.contains_key(x) {
                            continue 'outer;
                        }
                    }
                }
                *black_counts.entry(z).or_insert(0) += 1;
            }
        }
    }

    println!("[{}] black primitives", black_counts.len());

    let mut sorted_parts: Vec<(&&str, &usize)> = black_counts.iter().collect();
    sorted_parts.sort_by(|a, b| b.1.cmp(a.1));

    for i in 0..20 {
        let (id, count) = *sorted_parts.index(i);
        println!("[{}] {} : {}", i, id, count);
    }

	let mut chars_out = 0;
    for i in sorted_parts {
    	chars_out += i.0.len() + 2;
		if chars_out > 2048 {
			break;
		}    	
        println!("{}", i.0);
    }


}
