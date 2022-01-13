use std::collections::HashMap;

/// ```
/// use wordler::utils::count;
///
/// let count_aab = count(["a", "a", "b"]);
/// assert_eq!(count_aab["a"], 2);
/// assert_eq!(count_aab["b"], 1);
/// assert!(!count_aab.contains_key("c"));
/// ```
pub fn count<T, I>(i: I) -> HashMap<T, usize>
where
    I: std::iter::IntoIterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    i.into_iter().fold(HashMap::new(), |mut ac, e| {
        *(ac.entry(e).or_insert(0)) += 1;
        ac
    })
}
