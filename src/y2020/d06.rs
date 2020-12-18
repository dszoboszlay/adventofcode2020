use std::rc::Rc;
use crate::{Day, Part};

type Person = u32;
type Group = Vec<Person>;
type Plane = Vec<Group>;
type Input = Rc<Plane>;

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
    input.iter().map(|group| group.iter().fold(0, |acc, person| acc | person).count_ones()).sum::<u32>() as i64
}

fn p02(input: &Input) -> i64 {
    input.iter().map(|group| group.iter().fold(u32::MAX, |acc, person| acc & person).count_ones()).sum::<u32>() as i64
}

pub fn parse(s: String) -> Day {
    let mut input: Plane = Vec::new();
    let mut group: Group = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            input.push(group);
            group = Vec::new();
        } else {
            let mut person = 0;
            for c in line.bytes() {
                person += 1 << (c - ('a' as u8));
            }
            group.push(person);
        }
    }
    input.push(group);
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
