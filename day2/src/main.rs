use std::{
    error::Error,
    io::{stdin, BufRead},
};

#[derive(Debug, Clone, Copy)]
pub enum OPTIONS {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for OPTIONS {
    fn from(x: &str) -> Self {
        let Some(x) = x.bytes().take(1).next() else {
            panic!("Could not get first byte of str")
        };
        match x {
            b'A' => Self::Rock,
            b'B' => Self::Paper,
            b'C' => Self::Scissors,
            b'X' => Self::Rock,
            b'Y' => Self::Paper,
            b'Z' => Self::Scissors,
            _ => panic!("this option is undefined. Option: {x}"),
        }
    }
}

impl From<(&OPTIONS, &OUTCOME)> for OPTIONS {
    fn from((opponent, outcome): (&OPTIONS, &OUTCOME)) -> Self {
        use OPTIONS::*;
        use OUTCOME::*;
        match (opponent, outcome) {
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Loss) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Loss) => Paper,
            (x, Tie) => *x,
        }
    }
}

impl OPTIONS {
    fn from_int(x: i32) -> Option<Self> {
        use OPTIONS::*;
        match x {
            0 => Some(Rock),
            1 => Some(Paper),
            2 => Some(Scissors),
            _ => None,
        }
    }
}

impl From<OPTIONS> for i32 {
    fn from(x: OPTIONS) -> Self {
        use OPTIONS::*;
        match x {
            Rock => 0,
            Paper => 1,
            Scissors => 2,
        }
    }
}

#[derive(Debug)]
pub enum OUTCOME {
    Win,
    Loss,
    Tie,
}

impl From<(&OPTIONS, &OPTIONS)> for OUTCOME {
    fn from((opponent, you): (&OPTIONS, &OPTIONS)) -> Self {
        let op_val: i32 = i32::from(*opponent);
        let you_val: i32 = i32::from(*you);
        match you_val - op_val {
            -2 => Self::Win,
            -1 => Self::Loss,
            0 => Self::Tie,
            1 => Self::Win,
            2 => Self::Loss,
            _ => panic!("h m m : {}", you_val - op_val),
        }
    }
}

impl From<&str> for OUTCOME {
    fn from(x: &str) -> Self {
        use OUTCOME::*;
        let Some(x) = x.bytes().take(1).next() else {
            panic!("Could not get first byte of str")
        };
        match x {
            b'X' => Loss,
            b'Y' => Tie,
            b'Z' => Win,
            _ => panic!("this option is undefined. Option: {x}"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    use OUTCOME::*;
    let mut buf = String::new();
    let mut acc = Vec::new();
    let data = loop {
        let mut stdin = stdin().lock();
        let n = stdin.read_line(&mut buf)?;
        if n == 0 {
            break acc;
        }
        let (opponent, _you, outcome) = {
            let (opponent, you) = buf.split_at(1);
            (
                OPTIONS::from(opponent.trim()),
                OPTIONS::from(you.trim()),
                OUTCOME::from(you.trim()),
            )
        };
        let you = OPTIONS::from((&opponent, &outcome));
        println!("{outcome:?}");
        let value = {
            let outval = match outcome {
                Win => 6,
                Tie => 3,
                Loss => 0,
            };
            let choiceval = i32::from(you) + 1;
            outval + choiceval
        };
        acc.push(value);
        buf.clear()
    };
    let total = data.iter().fold(0, |acc, x| acc + x);
    println!("Total: {total}");
    Ok(())
}
