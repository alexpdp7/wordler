use wordler::*;

fn main() {
    println!("{}", best_quality(&dict::<5>()).iter().collect::<String>());
}
