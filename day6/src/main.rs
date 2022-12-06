use day6;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let star_1 = day6::find_start(&input, 4).unwrap();
    let star_2 = day6::find_start(&input, 14).unwrap();
    println!("Star one: {star_1}\nStar two: {star_2}");
}
