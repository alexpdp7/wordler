use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CharScore {
    Correct,
    Misplaced,
    NotFound,
}

fn evaluate<const N: usize>(guess: [char; N], solution: [char; N]) -> [CharScore; N] {
    let mut result: [CharScore; N] = [CharScore::NotFound; N];

    let mut x: HashMap<char, (Vec<usize>, HashSet<usize>)> = HashMap::new();
    for (i, (g, s)) in guess
        .iter()
        .zip(solution.iter())
        .enumerate()
        .collect::<Vec<_>>()
    {
        (*x.entry(*g).or_insert((Vec::new(), HashSet::new())))
            .0
            .push(i);
        (*x.entry(*s).or_insert((Vec::new(), HashSet::new())))
            .1
            .insert(i);
    }

    let mut y: HashMap<char, (Vec<usize>, usize)> = HashMap::new();
    for (c, (gps, sps)) in &x {
        let mut ngps: Vec<usize> = Vec::new();
        let mut nsps: usize = 0;
        for i in (Range {
            start: 0,
            end: std::cmp::min(gps.len(), sps.len()),
        }) {
            if sps.contains(&gps[i]) {
                result[gps[i]] = CharScore::Correct;
            } else {
                ngps.push(gps[i]);
                nsps += 1;
            }
        }
        y.insert(*c, (ngps, nsps));
    }

    for (gps, sps) in y.values() {
        for i in (Range {
            start: 0,
            end: std::cmp::min(gps.len(), *sps),
        }) {
            result[gps[i]] = CharScore::Misplaced;
        }
    }

    result
}

fn is_lowercase_ascii(s: &str) -> bool {
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
    println!(
        "{:?}",
        evaluate(['a', 'b', 'c', 'd', 'e'], ['a', 'b', 'd', 'c', 'f'])
    );

    println!(
        "{:?}",
        evaluate(['b', 'b', 'b', 'a', 'a'], ['a', 'a', 'b', 'b', 'b'])
    );
}
