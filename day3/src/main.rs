use day6;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let x = day6::find_start(&input, 4).unwrap();
    println!("{x}");
}
