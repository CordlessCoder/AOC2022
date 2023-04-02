#![feature(iter_array_chunks)]
use std::{fs::File, io::Read};

fn star2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks()
        .fold(0, |sum, line: [&str; 3]| {
            let groups = line.map(|line| str_to_mask(line));
            let intersection = groups
                .into_iter()
                .reduce(|intersection, mask| intersection & mask)
                // SAFETY: Because of array_chunks, the iterator will HAVE to have 3 elements
                .unwrap();
            let offset = intersection.trailing_zeros();

            let priority = offset_to_priority(offset);
            sum + priority
        })
}

fn main() {
    let mut input = String::with_capacity(128);
    File::open("input")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read file");
    let star1 = star1(&input);
    println!("{star1}");
    let star2 = star2(&input);
    println!("{star2}");
}

fn star1(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let half = line.len() / 2;
        let (first, second) = line.split_at(half);
        let (first, second) = (str_to_mask(first), str_to_mask(second));
        let intersection = first & second;
        let offset = intersection.trailing_zeros();
        let priority = offset_to_priority(offset);
        sum + priority
    })
}

fn str_to_mask(s: &str) -> u64 {
    let mut mask = 0;
    s.bytes().map(|x| x - b'A').for_each(|x| mask |= 1 << x);
    mask
}

fn offset_to_priority(offset: u32) -> u32 {
    if offset < 26 {
        // If the letter is uppercase
        offset + 27
    } else {
        // If the letter is lowercase
        offset - 31
    }
}
