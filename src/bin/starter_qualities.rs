use wordler::*;

fn main() {
    let dict = dict::<5>();
    for w in &dict {
        println!("{}\t{}", quality(&dict, *w), w.iter().collect::<String>());
    }
}
