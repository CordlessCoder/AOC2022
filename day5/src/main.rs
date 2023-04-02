#![feature(iter_array_chunks)]
use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read the contents of the input file");
    let star1 = star1(&input);
    println!("{star1}");
    let star2 = star2(&input);
    println!("{star2}");
}

fn star1(input: &str) -> String {
    let mut lines = input.lines();
    // Parse crates header
    let mut crates: Vec<Vec<u8>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.bytes().step_by(4).all(|b| b == b' ') {
            // If we reached the end of the crate header
            // Get rid of trailing newline
            lines.next();
            break;
        }
        line.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, letter)| *letter != b' ')
            .for_each(|(idx, letter)| {
                if let Some(stack) = crates.get_mut(idx) {
                    // println!("pushed {} to stack {idx}", letter as char);
                    stack.push(letter)
                } else {
                    // This stack has not been created yet, extend the crates vector with as many
                    // new stacks as needed
                    // println!("extended stacks of len {} for idx {idx}", crates.len());
                    crates.extend(std::iter::repeat(Vec::new()).take(idx - crates.len()));
                    crates.push(vec![letter]);
                    // println!("len after extend: {}", crates.len());
                }
            });
    }
    // Put the crates into the correct order
    crates.iter_mut().for_each(|stack| stack.reverse());
    let mut tempvec = Vec::with_capacity(4);
    lines.for_each(|line| {
        let mut words = line.split(' ').skip(1).step_by(2);
        let [amount, from, to]: [usize; 3] = [(); 3].map(|_| {
            words
                .next()
                .expect("Failed to extract data from line. Is there a trailing newline?")
                .parse()
                .expect("Failed to parse input as number")
        });
        // dbg!(from);
        // dbg!(crates.len());
        let stack_from = crates.get_mut(from - 1).unwrap();
        let len = stack_from.len();
        stack_from
            .drain(len - amount..)
            .for_each(|cr| tempvec.push(cr));
        let stack_to = crates.get_mut(to - 1).unwrap();
        tempvec.iter().rev().for_each(|&cr| stack_to.push(cr));
        tempvec.clear()
    });
    get_top(&crates)
}

fn star2(input: &str) -> String {
    let mut lines = input.lines();
    // Parse crates header
    let mut crates: Vec<Vec<u8>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.bytes().step_by(4).all(|b| b == b' ') {
            // If we reached the end of the crate header
            // Get rid of trailing newline
            lines.next();
            break;
        }
        line.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, letter)| *letter != b' ')
            .for_each(|(idx, letter)| {
                if let Some(stack) = crates.get_mut(idx) {
                    // println!("pushed {} to stack {idx}", letter as char);
                    stack.push(letter)
                } else {
                    // This stack has not been created yet, extend the crates vector with as many
                    // new stacks as needed
                    // println!("extended stacks of len {} for idx {idx}", crates.len());
                    crates.extend(std::iter::repeat(Vec::new()).take(idx - crates.len()));
                    crates.push(vec![letter]);
                    // println!("len after extend: {}", crates.len());
                }
            });
    }
    // Put the crates into the correct order
    crates.iter_mut().for_each(|stack| stack.reverse());
    let mut tempvec = Vec::with_capacity(4);
    lines.for_each(|line| {
        let mut words = line.split(' ').skip(1).step_by(2);
        let [amount, from, to]: [usize; 3] = [(); 3].map(|_| {
            words
                .next()
                .expect("Failed to extract data from line. Is there a trailing newline?")
                .parse()
                .expect("Failed to parse input as number")
        });
        // dbg!(from);
        // dbg!(crates.len());
        let stack_from = crates.get_mut(from - 1).unwrap();
        let len = stack_from.len();
        stack_from
            .drain(len - amount..)
            .for_each(|cr| tempvec.push(cr));
        let stack_to = crates.get_mut(to - 1).unwrap();
        stack_to.extend_from_slice(&tempvec);
        tempvec.clear()
    });
    get_top(&crates)
}

fn get_top(crates: &[Vec<u8>]) -> String {
    let mut out = String::with_capacity(crates.len());
    crates
        .iter()
        .filter_map(|stack| stack.last())
        .for_each(|&top| out.push(top as char));
    out
}
