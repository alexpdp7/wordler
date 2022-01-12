use wordler::{dict, evaluate, pretty_eval};

fn main() {
    print!(
        "{:?}",
        dict::<5>()
            .iter()
            .map(|w| pretty_eval(&evaluate(*w, ['a', 'b', 'c', 'd', 'e'])))
            .collect::<Vec<String>>()
    );
}
