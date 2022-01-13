use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharScore {
    Correct,
    Misplaced,
    NotFound,
}

impl std::fmt::Display for CharScore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CharScore::Correct => "O",
                CharScore::Misplaced => "X",
                CharScore::NotFound => ".",
            }
        )
    }
}

pub fn pretty_eval(evaluation: &[CharScore]) -> String {
    evaluation.iter().map(|cs| format!("{}", cs)).collect()
}

/// ```
/// use wordler::{pretty_eval, evaluate};
///
/// assert_eq!(pretty_eval(&evaluate(['a', 'b', 'c', 'd', 'e'], ['a', 'b', 'd', 'c', 'f'])), "OOXX.");
/// assert_eq!(pretty_eval(&evaluate(['b', 'b', 'b', 'a', 'a'], ['a', 'a', 'b', 'b', 'b'])), "XXOXX");
/// ```
pub fn evaluate<const N: usize>(guess: [char; N], solution: [char; N]) -> [CharScore; N] {
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

fn count<T: Eq + std::hash::Hash>(i: &mut dyn Iterator<Item = T>) -> HashMap<T, usize> {
    i.fold(HashMap::new(), |mut ac, e| {
        *(ac.entry(e).or_insert(0)) += 1;
        ac
    })
}

fn word_to_count_evaluations<const N: usize>(
    dict: &HashSet<[char; N]>,
    word: [char; N],
) -> HashMap<[CharScore; N], usize> {
    count(&mut dict.iter().map(|w| evaluate(*w, word)))
}

fn count_to_quality<const N: usize>(count: HashMap<[CharScore; N], usize>) -> usize {
    count.values().map(|c| c * c).sum()
}

pub fn quality<const N: usize>(dict: &HashSet<[char; N]>, word: [char; N]) -> usize {
    count_to_quality(word_to_count_evaluations(dict, word))
}
