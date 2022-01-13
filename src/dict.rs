use std::collections::HashSet;
use std::io::BufRead;

/// Loads words from `/usr/share/dict/words`
///
/// ```
/// use wordler::dict::dict;
///
/// assert!(dict::<5>().contains(&['h', 'e', 'l', 'l', 'o']));
/// assert!(dict::<6>().contains(&['h', 'e', 'l', 'l', 'o', 's']));
/// ```
pub fn dict<const N: usize>() -> HashSet<[char; N]> {
    std::io::BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() == N)
        .filter(|l| is_lowercase_ascii(l))
        .map(to_char_array)
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lowercase_ascii() {
        assert!(is_lowercase_ascii("abcde"));
        assert!(!is_lowercase_ascii("Abcde"));
        assert!(!is_lowercase_ascii("ábcde"));
    }

    #[test]
    fn test_to_char_array() {
        assert_eq!(
            to_char_array::<5>("abcde".to_string()),
            ['a', 'b', 'c', 'd', 'e']
        )
    }
}
