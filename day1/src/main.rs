use std::{
    error::Error,
    io::{stdin, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let mut acc = Vec::new();
    let data = loop {
        let mut stdin = stdin().lock();
        let n = stdin.read_line(&mut buf)?;
        match buf.trim().parse::<i32>() {
            Ok(n) => {
                if let Some(value) = acc.last_mut() {
                    *value += n
                } else {
                    acc.push(n)
                };
            }
            Err(_) => acc.push(0),
        };
        if n == 0 {
            break acc;
        }
        buf.clear()
    };
    let elves = data;
    let top_one = elves.iter().enumerate().max_by_key(|(_, &x)| x).unwrap();
    let mut elves = elves.clone();
    elves.sort();
    let top3 = elves.iter().rev().take(3).fold(0, |acc, x| acc + x);
    println!("{elves:?}\n{top_one:?}\n{top3}");
    Ok(())
}
