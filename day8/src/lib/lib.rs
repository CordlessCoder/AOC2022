type DigitSet = u8;

#[derive(Debug)]
pub enum Direction {
    Horiz,
    Vert,
}

#[derive(Debug, Clone, Copy)]
pub enum Tree {
    Unknown(u8),       // Used for initializing trees when parsing
    Visible(DigitSet), // the u8 is used to store the max height of all possible trees that could occlude
    // it, which is always the height of the original Unknown variant of the tree as it cannot be
    // occluded by smaller trees
    // original height of the tree as it cannot
    Invisible(DigitSet), // Used for storing the height of the tallest tree that the given tree was
                         // occluded with
}

impl Tree {
    #[inline]
    pub fn solve(
        grid: &mut [[Tree; 99]; 99], // The 2d array used to look up the trees that are in the same
        // row/column
        position: (usize, usize), // The position of the tree we're solving for
        size: usize,              // length of side
    ) {
        use Tree::*;
        let Unknown(height) = grid[position.0][position.1] else {
            return ()
        };
        grid[position.0][position.1] = Self::decide(position, size, height, &grid)
    }
    // pub fn calc_dir_min_height(cmp_height: u8, dir: &Direction) -> u8 {}
    #[inline]
    fn decide(
        (x, y): (usize, usize),
        size: usize,
        self_height: u8,
        grid: &[[Tree; 99]; 99], // The 2d array used to look up the trees that are in the same
    ) -> Self {
        use Direction::*;
        let mut buf = [0u8; 4];
        let mut val = 0;
        let mut stop: bool;
        for (x, y, dir) in (0..y).rev().map(|y| (x, y, Vert)) {
            // iterating over all trees in that direction, closest to furthest.
            (val, stop) = Self::calc(val, &grid, x, y, dir);
            if stop {
                break;
            }
        }
        buf[0] = val;
        let mut val = 0;
        let mut stop: bool;
        for (x, y, dir) in (y + 1..size).map(|y| (x, y, Vert)) {
            // iterating over all trees in that direction, closest to furthest.
            (val, stop) = Self::calc(val, &grid, x, y, dir);
            if stop {
                break;
            }
        }
        buf[1] = val;
        let mut val = 0;
        let mut stop: bool;
        for (x, y, dir) in (0..x).rev().map(|x| (x, y, Horiz)) {
            // iterating over all trees in that direction, closest to furthest.
            (val, stop) = Self::calc(val, &grid, x, y, dir);
            if stop {
                break;
            }
        }
        buf[2] = val;
        let mut val = 0;
        let mut stop: bool;
        for (x, y, dir) in (x + 1..size).map(|x| (x, y, Horiz)) {
            // iterating over all trees in that direction, closest to furthest.
            (val, stop) = Self::calc(val, &grid, x, y, dir);
            if stop {
                break;
            }
        }
        buf[3] = val;
        let (horiz_noself, vert_noself) = (buf[2].min(buf[3]), buf[0].min(buf[1]));
        let visible = self_height > horiz_noself.min(vert_noself);
        let (horiz, vert) = (horiz_noself.max(self_height), vert_noself.max(self_height));
        if visible {
            Tree::Visible(DigitSet::pack(horiz, vert))
        } else {
            Tree::Invisible(DigitSet::pack(horiz, vert))
        }
    }
    fn calc(init: u8, grid: &[[Tree; 99]; 99], x: usize, y: usize, dir: Direction) -> (u8, bool) {
        use Direction::*;
        use Tree::*;
        let mut out: u8 = init;
        // iterating over all trees in that direction, closest to furthest.
        let tree = grid[x][y];
        match tree {
            Unknown(height) => {
                out = out.max(height & 0b00001111u8);
                return (out, false);
            }
            Visible(height) => {
                let height = match dir {
                    Horiz => DigitSet::unpack(&height).0,
                    Vert => DigitSet::unpack(&height).1,
                };
                out = out.max(height);
            }
            Invisible(height) => {
                let height = match dir {
                    Horiz => DigitSet::unpack(&height).0,
                    Vert => DigitSet::unpack(&height).1,
                };
                out = out.max(height);
            }
        };
        (out, true)
    }
    pub fn set(&self, horiz: u8, vert: u8) -> Self {
        let value = DigitSet::pack(horiz, vert);
        match self {
            Tree::Visible(_) => Tree::Visible(value),
            Tree::Invisible(_) => Tree::Invisible(value),
            Tree::Unknown(_) => Tree::Unknown(value),
        }
    }
    pub fn get(&self) -> (u8, u8) {
        DigitSet::unpack(match self {
            Tree::Visible(value) => value,
            Tree::Invisible(value) => value,
            Tree::Unknown(value) => value,
        })
    }
}

pub trait PackedInt {
    fn pack(horiz: u8, vert: u8) -> u8;
    fn unpack(packed: &u8) -> (u8, u8);
}

impl PackedInt for DigitSet {
    fn pack(horiz: u8, vert: u8) -> u8 {
        let mask = 0b00001111u8;
        ((horiz & mask) << 4) | (vert & mask)
    }
    fn unpack(packed: &u8) -> (u8, u8) {
        let mask = 0b00001111u8;
        (((packed) >> 4), (packed & mask))
    }
}

const SIZE: usize = 99;
pub fn parse(input: &str) -> [[Tree; SIZE]; SIZE] {
    let mut grid = [[Tree::Unknown(0); SIZE]; SIZE];
    input
        .as_bytes()
        .split(|&c| c == b'\n')
        .enumerate()
        .for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| {
                let height = c - b'0';
                if x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1 {
                    grid[x][y] = Tree::Visible(DigitSet::pack(height, height))
                } else {
                    grid[x][y] = Tree::Unknown(height)
                }
            })
        });
    grid
}

pub fn solve_parsed(mut grid: [[Tree; 99]; 99]) -> usize {
    for depth in 1..=SIZE / 2 {
        // How far away should we stay from the edge, starting at 1 as we
        // always know that all trees there are visible
        let last = SIZE - depth;
        for y in depth..last {
            let depthb = SIZE - depth - 1;
            if (depth + 1..last - 1).contains(&y) {
                for (x, y) in [(depth, y), (depthb, SIZE - y - 1), (y, depth), (y, depthb)] {
                    // iterating over all trees that are `depth` far away from the edge at the left and
                    // right
                    Tree::solve(&mut grid, (x, y), SIZE);
                }
            } else {
                for (x, y) in [(depth, y), (depthb, SIZE - y - 1)] {
                    // iterating over all trees that are `depth` far away from the edge at the left and
                    // right
                    Tree::solve(&mut grid, (x, y), SIZE);
                }
            }
        }
    }
    let visible = grid
        .iter()
        .map(|x| {
            x.iter()
                .filter(|&x| match x {
                    Tree::Visible(_) => true,
                    _ => false,
                })
                .count()
        })
        .reduce(|acc, x| acc + x)
        .unwrap();
    visible
}

pub fn solve(input: &str) -> usize {
    let grid = parse(input);
    solve_parsed(grid)
}
