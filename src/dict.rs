use std::collections::HashSet;
use std::io::BufRead;

fn is_lowercase_ascii(s: &str) -> bool {
    s.as_bytes().iter().all(|c| b'a' <= *c && *c <= b'z')
}

fn to_char_array<const N: usize>(s: String) -> [char; N] {
    let mut result = ['.'; N];
    for (i, c) in s.chars().enumerate() {
        result[i] = c;
    }
    result
}

pub fn dict<const N: usize>() -> HashSet<[char; N]> {
    std::io::BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() == N)
        .filter(|l| is_lowercase_ascii(l))
        .map(to_char_array)
        .collect()
}
