use wordler::{dict, quality};

fn main() {
    let dict5 = dict::<5>();
    for w in &dict5 {
        println!("{} {}", quality(&dict5, *w), w.iter().collect::<String>());
    }
}
