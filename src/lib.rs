pub mod dict;
pub mod score;
pub mod utils;

use crate::score::{evaluate, CharScore};
use crate::utils::count;
use std::collections::{HashMap, HashSet};

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
