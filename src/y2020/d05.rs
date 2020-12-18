use std::rc::Rc;
use crate::{Day, Part};

type Input = Rc<Vec<u16>>;

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

fn p01(input: &Input) -> i64 {
    input[input.len() - 1] as i64
}

fn p02(input: &Input) -> i64 {
    let mut i = input[0] + 2;
    for id in input.iter() {
        if *id == i { return (id - 1) as i64; }
        i = id + 2;
    };
    -1
}

pub fn parse(s: String) -> Day {
    let mut input: Vec<u16> = s.lines().map(|line| {
        let mut id = 0;
        for c in line.chars() {
            id <<= 1;
            if c == 'B' || c == 'R' { id += 1; }
        };
        id
    }).collect();
    input.sort();
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
