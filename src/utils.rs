use std::collections::HashMap;

pub fn count<T: Eq + std::hash::Hash>(i: &mut dyn Iterator<Item = T>) -> HashMap<T, usize> {
    i.fold(HashMap::new(), |mut ac, e| {
        *(ac.entry(e).or_insert(0)) += 1;
        ac
    })
}
