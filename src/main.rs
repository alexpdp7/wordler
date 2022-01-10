use std::collections::HashSet;
use std::io::BufRead;

fn is_lowercase_ascii(s: &String) -> bool {
    s.as_bytes().iter().all(|c| b'a' <= *c && *c <= b'z')
}

fn dict() -> HashSet<String> {
    std::io::BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() == 5)
        .filter(|l| is_lowercase_ascii(l))
        .collect()
}

fn main() {
    println!("{:?}", dict());
}
