use std::rc::Rc;
use crate::{Day, Part};

type Input = Rc<Vec<usize>>;

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
    let mut memory = [0usize; 2020];
    for (i, &current) in input.iter().enumerate() {
        memory[current] = i + 1;
    }
    
    let mut i = input.len();
    let mut last = *input.last().unwrap();
    loop {
        let current = if memory[last] == 0 { 0 } else { i - memory[last] };
        memory[last] = i;
        i += 1;
        if i == 2020 {
            return current as i64
        } else {
            last = current;
        }
    }
}

fn p02(input: &Input) -> i64 {
    let mut memory = vec![0usize; 30000000];
    for (i, &current) in input.iter().enumerate() {
        memory[current] = i + 1;
    }
    
    let mut i = input.len();
    let mut last = *input.last().unwrap();
    loop {
        let current = if memory[last] == 0 { 0 } else { i - memory[last] };
        memory[last] = i;
        i += 1;
        if i == 30000000 {
            return current as i64
        } else {
            last = current;
        }
    }
}

pub fn parse(s: String) -> Day {
    let input = s.split(',').map(|n| n.parse().unwrap()).collect();    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
