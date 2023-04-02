use day8::solve;
use std::fs;
fn main() {
    const FILENAME: &str = "input";
    let input =
        fs::read_to_string(FILENAME).expect(&format!("Couldn't open input file `{FILENAME}`"));
    let star1 = solve(&input);
    println!("Star 1: {star1}")
}
