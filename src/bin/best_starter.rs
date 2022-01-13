use wordler::dict::dict;
use wordler::quality;

fn main() {
    let dict = dict::<5>();
    println!(
        "{}",
        dict.iter()
            .min_by_key(|w| quality(&dict, **w))
            .unwrap()
            .iter()
            .collect::<String>()
    );
}
