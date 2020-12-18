use std::rc::Rc;
use std::result::Result;
use std::str::FromStr;
use crate::{Day, Part};

#[derive(Debug, PartialEq)]
struct Map {
    w: usize,
    h: usize,
    trees: Vec<u8>
}

impl Map {
    fn has_tree(&self, x: usize, y: usize) -> bool {
        self.trees[y * (self.w + 1) + x] == '#' as u8
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h = s.lines().count();
        let w = s.lines().next().unwrap().len();

        Result::Ok(Map {
            w: w,
            h: h,
            trees: s.bytes().collect()
        })
    }
}

type Input = Rc<Map>;

struct Part1 {
    input: Input
}

impl Part for Part1 {
    fn solve(&self) -> i64 { p01(&self.input) }
}

struct Part2 {
    input: Input
}

impl Part for Part2 {
    fn solve(&self) -> i64 { p02(&self.input) }
}

fn slope(input: &Input, dx: usize, dy: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut cnt = 0;

    while y < input.h {
        if input.has_tree(x, y) { cnt += 1; }
        x = x + dx;
        y = y + dy;
        if x >= input.w { x -= input.w; }
    }

    return cnt
}

fn p01(input: &Input) -> i64 {
    slope(input, 3, 1)
}

fn p02(input: &Input) -> i64 {
    slope(input, 1, 1) * 
    slope(input, 3, 1) * 
    slope(input, 5, 1) * 
    slope(input, 7, 1) *
    slope(input, 1, 2) 
}

pub fn parse(s: String) -> Day {
    let input: Map = s.parse().unwrap();
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
