use wordler::{evaluate, pretty_eval};

fn main() {
    println!(
        "{}",
        pretty_eval(&evaluate(
            ['a', 'b', 'c', 'd', 'e'],
            ['a', 'b', 'd', 'c', 'f']
        ))
    );

    println!(
        "{}",
        pretty_eval(&evaluate(
            ['b', 'b', 'b', 'a', 'a'],
            ['a', 'a', 'b', 'b', 'b']
        ))
    );
}
