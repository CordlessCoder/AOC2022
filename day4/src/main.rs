use std::{fs::File, io::Read, num::ParseIntError, str::FromStr};

fn main() {
    let mut input = String::new();
    File::open("input")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read the contents of input file");
    let star1 = star1(&input);
    println!("{star1}");
    let star2 = star2(&input);
    println!("{star2}")
}

fn star1(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line
                .split_once(',')
                .expect("Failed to find a comma in line of input. Is there a trailing newline?");
            let out: (SectionRange, SectionRange) =
                (first.parse().unwrap(), second.parse().unwrap());
            out
        })
        .fold(0, |acc, (lhs, rhs)| {
            acc + lhs.contains_or_is_contained(rhs) as u16
        })
}

fn star2(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line
                .split_once(',')
                .expect("Failed to find a comma in line of input. Is there a trailing newline?");
            let out: (SectionRange, SectionRange) =
                (first.parse().unwrap(), second.parse().unwrap());
            out
        })
        .fold(0, |acc, (lhs, rhs)| acc + lhs.overlap(rhs) as u16)
}

#[derive(Debug, Clone, Copy)]
struct SectionRange {
    start: u8,
    end: u8,
}

impl SectionRange {
    pub fn new(start: u8, end: u8) -> Self {
        SectionRange { start, end }
    }
    pub fn contains(self, other: SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    pub fn is_in(self, other: SectionRange) -> bool {
        self.start >= other.start && self.end <= other.end
    }
    pub fn contains_or_is_contained(self, other: SectionRange) -> bool {
        self.contains(other) || self.is_in(other)
    }
    pub fn overlap(self, other: SectionRange) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

impl FromStr for SectionRange {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once('-').ok_or(SectionParseError::NoMinusFound)?;
        let (lhs, rhs) = (
            lhs.parse()
                .map_err(|err| SectionParseError::ParseError(err))?,
            rhs.parse()
                .map_err(|err| SectionParseError::ParseError(err))?,
        );
        Ok(SectionRange::new(lhs, rhs))
    }
}

#[derive(Debug)]
pub enum SectionParseError {
    NoMinusFound,
    ParseError(ParseIntError),
}
